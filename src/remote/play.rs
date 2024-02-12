use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ConnectionStatus {
    Uninitialized,
    Open,
    Closed,
}

#[derive(Debug)]
struct Connection {
    address: String,
    room: server::RoomCode,
    player: bool,
    pub status: Cell<ConnectionStatus>,
}

impl Connection {
    async fn login(mut self) -> Result<Self, String> {
        let bar = indicatif::ProgressBar::new_spinner().with_message("Connecting to the server...");
        bar.enable_steady_tick(Duration::from_millis(300));

        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/chess/login", self.address))
            .body(json!(server::LoginRequest { room: self.room }).to_string())
            .send()
            .await
            .map_err(|e| e.to_string())?;

        bar.finish_with_message(format!("Connected to {}!", self.address));

        match serde_json::from_str(&res.text().await.map_err(|e| e.to_string())?).map(
            |res: server::LoginResponse| {
                if res.player {
                    println!(
                        "{} You are {}!",
                        style("Welcome!").green(),
                        style("White").on_white().black()
                    );
                } else {
                    println!(
                        "{} You are {}!",
                        style("Welcome!").green(),
                        style("Black").on_black().white()
                    );
                }
                res
            },
        ) {
            Ok(res) => {
                self.player = res.player;
                Ok(self)
            }
            Err(_e) => Err("Failed to join the game".to_string()),
        }
    }

    async fn play(&self, cmd: String, board: &game::Board) -> Result<(), String> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/chess/play", self.address))
            .body(
                json!(server::CommandRequest {
                    room: self.room,
                    player: self.player,
                    cmd,
                    board: board.serialize()
                })
                .to_string(),
            )
            .send()
            .await
            .map_err(|e| e.to_string())?;
        match res.status() {
            StatusCode::OK => Ok(()),
            _ => Err(res.text().await.map_err(|e| e.to_string())?),
        }
    }

    async fn query(&self) -> Result<Option<server::QueryResponse>, String> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/chess/query", self.address))
            .body(
                json!(server::QueryRequest {
                    room: self.room,
                    player: self.player
                })
                .to_string(),
            )
            .send()
            .await
            .map_err(|e| e.to_string())?;
        match res.status() {
            StatusCode::OK => {
                match serde_json::from_str(&res.text().await.map_err(|e| e.to_string())?) {
                    Ok(res) => Ok(Some(res)),
                    Err(_e) => Err("Failed to query the game".to_string()),
                }
            }
            StatusCode::NOT_FOUND => Err(res.text().await.map_err(|e| e.to_string())?),
            _ => Ok(None),
        }
    }

    async fn logout(&self) -> Result<(), String> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/chess/logout", self.address))
            .body(json!(server::LogoutRequest { room: self.room }).to_string())
            .send()
            .await
            .map_err(|e| e.to_string())?;
        self.status.set(ConnectionStatus::Closed);
        match res.status() {
            StatusCode::OK => Ok(()),
            _ => Err(res.text().await.map_err(|e| e.to_string())?),
        }
    }

    async fn is_ok(&self) -> Result<bool, String> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/chess/is_ok", self.address))
            .body(json!(server::IsOkRequest { room: self.room }).to_string())
            .send()
            .await
            .map_err(|e| e.to_string())?;
        match res.status() {
            StatusCode::OK => {
                let response: server::IsOkResponse =
                    match serde_json::from_str(&res.text().await.map_err(|e| e.to_string())?) {
                        Ok(res) => res,
                        Err(_e) => return Err("Failed to query the game".to_string()),
                    };
                Ok(response.ok)
            }
            _ => Err(res.text().await.map_err(|e| e.to_string())?),
        }
    }

    async fn log_back(&self) -> Result<String, ()> {
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/chess/log_back", self.address))
            .body(
                json!(server::LogBackRequest {
                    room: self.room,
                    player: self.player
                })
                .to_string(),
            )
            .send()
            .await
            .map_err(|_e| ())?;
        match res.status() {
            StatusCode::OK => {
                let response: server::LogBackResponse =
                    match serde_json::from_str(&res.text().await.map_err(|_e| ())?) {
                        Ok(res) => res,
                        Err(_e) => return Err(()),
                    };
                Ok(response.board)
            }
            _ => Err(()),
        }
    }

    fn build() -> Self {
        let address = dialoguer::Input::new()
            .with_prompt("Enter the server address")
            .default(CONFIG.address.clone())
            .interact()
            .unwrap();
        let room: String = dialoguer::Input::new()
            .with_prompt("Enter the room identifier(e.g. \"my-chess-room\")")
            .interact()
            .unwrap();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        hasher.write(room.as_bytes());
        let room = hasher.finish();

        Self {
            address,
            room,
            player: false,
            status: Cell::new(ConnectionStatus::Uninitialized),
        }
    }
}

async fn play_remotely_with(connection: Connection, mut board: game::Board) -> Connection {
    let error = style("Error").red().bold();
    let terminate = style("Terminating due to error").red().bold();

    if connection.status.get() == ConnectionStatus::Uninitialized {
        let bar =
            indicatif::ProgressBar::new_spinner().with_message("Waiting for opponent to join...");
        bar.enable_steady_tick(Duration::from_millis(300));

        loop {
            tokio::time::sleep(Duration::from_millis(300)).await;

            let q = connection.is_ok().await;
            if let Err(err) = q {
                println!("{} {}", terminate, err);
                return connection;
            }

            if q.unwrap() {
                break;
            }

            tokio::time::sleep(Duration::from_millis(700)).await;
        }

        bar.finish_with_message("Opponent joined!".to_string());

        connection.status.set(ConnectionStatus::Open);
    }

    let mut is_turn = connection.player;
    let mut err: Option<String> = None;

    'game_loop: while board.status == game::Status::Playing {
        if is_turn {
            println!("{}", board);
        } else {
            println!("{:#}", board);
        }

        if let Some(msg) = err {
            println!("{}: {}", error, msg);
            err = None;
        }

        let (command, pronoun, player_str) = if is_turn {
            let str = dialoguer::Input::<String>::new()
                .with_prompt("Command")
                .interact()
                .unwrap();
            let command = util::parse_raw(str.clone());

            (command, "You", Some(str))
        } else {
            let mut query;
            let bar = indicatif::ProgressBar::new_spinner()
                .with_message("Waiting for opponent to move...");
            bar.enable_steady_tick(Duration::from_millis(300));

            loop {
                tokio::time::sleep(Duration::from_millis(300)).await;

                let q = connection.query().await;
                if let Err(err) = q {
                    println!("{} {}", terminate, err);
                    break 'game_loop;
                }
                query = q.unwrap();

                if query.is_some() {
                    break;
                }

                tokio::time::sleep(Duration::from_millis(700)).await;
            }
            let query = query.unwrap();
            bar.finish_and_clear();
            if !query.cmd.starts_with("chat") {
                println!("Opponent: {}", query.cmd);
            }

            let command = util::parse_raw(query.cmd);

            (command, "Opponent", None)
        };

        if board.draw_offer && command != util::Command::Draw {
            println!("Draw offer has been declined!");

            if let Some(str) = player_str {
                let play = connection.play(str, &board).await;
                if let Err(err) = play {
                    println!("{} {}", terminate, err);
                    break 'game_loop;
                }
            }

            is_turn = !is_turn;

            board.decline_draw();
        } else {
            match command {
                util::Command::Chess(str) => {
                    let notation = board.translate(&str);
                    if let Ok(notation) = notation {
                        let next = board.perform(notation);
                        if next.is_none() {
                            err = Some("Invalid move! This leads to a check!".to_string());
                        } else {
                            is_turn = !is_turn;

                            board = next.unwrap();

                            if let Some(str) = player_str {
                                let play = connection.play(str, &board).await;
                                if let Err(err) = play {
                                    println!("{} {}", terminate, err);
                                    break 'game_loop;
                                }
                            }
                        }
                    } else {
                        err = Some(format!("{}", notation.unwrap_err()));
                    }
                }
                util::Command::Resign => {
                    println!("{} resigned!", pronoun);

                    // This is technically not necessary because the game is ending soon, but it's a good practice to keep the game state consistent
                    is_turn = !is_turn;

                    board.resign();

                    if let Some(str) = player_str {
                        let play = connection.play(str, &board).await;
                        if let Err(err) = play {
                            println!("{} {}", terminate, err);
                            break 'game_loop;
                        }
                    }
                }
                util::Command::Draw => {
                    is_turn = !is_turn;

                    board.draw();

                    if let Some(str) = player_str {
                        let play = connection.play(str, &board).await;
                        if let Err(err) = play {
                            println!("{} {}", terminate, err);
                            break 'game_loop;
                        }
                    }
                }
                util::Command::Chat(str) => {
                    if let Some(str) = player_str {
                        let play = connection.play(str, &board).await;
                        if let Err(err) = play {
                            println!("{} {}", terminate, err);
                            break 'game_loop;
                        }
                    } else {
                        println!("{}: {}", style("Chat").bold(), str);
                        println!("Continue...");
                        let term = console::Term::stdout();
                        term.read_key().unwrap();
                    }
                }
            }
        }
        println!();
    }

    if board.status != game::Status::Playing {
        println!("{}", board);

        {
            let bar =
                indicatif::ProgressBar::new_spinner().with_message("Waiting for game to finish...");
            bar.enable_steady_tick(Duration::from_millis(300));

            let number: u64 = rand::random();
            tokio::time::sleep(Duration::from_millis(3000 + number % 2000)).await;

            bar.finish_and_clear();
        }

        let _ = connection.logout().await;
    }

    connection
}

async fn play_remotely_with_any(connection: Option<Connection>) -> Option<Connection> {
    let (connection, board) = match connection {
        None => (
            {
                let connection = Connection::build().login().await;
                if let Err(err) = connection {
                    println!("{} {}", style("Error").red().bold(), err);
                    return None;
                }
                connection.unwrap()
            },
            game::Board::new(),
        ),
        Some(connection) => match connection.status.get() {
            ConnectionStatus::Uninitialized => {
                let connection = connection.login().await;
                if let Err(err) = connection {
                    println!("{} {}", style("Error").red().bold(), err);
                    return None;
                }
                (connection.unwrap(), game::Board::new())
            }
            _ => {
                let bar = indicatif::ProgressBar::new_spinner()
                    .with_message("Attempting to reconnect to the server...");
                bar.enable_steady_tick(Duration::from_millis(300));
                let start_time = std::time::Instant::now();
                let board = loop {
                    match connection.log_back().await {
                        Ok(board) => {
                            bar.finish_with_message("Reconnected to the server!".to_string());
                            break board;
                        }
                        Err(_) => {
                            tokio::time::sleep(Duration::from_millis(300)).await;
                        }
                    }
                    if start_time.elapsed() > Duration::from_secs(15) {
                        println!("{}: Reconnection timed out.", style("Error").red().bold());
                        let _ = connection.logout().await;
                        return Some(connection);
                    }
                };
                (connection, game::Board::deserialize(board).unwrap())
            }
        },
    };
    Some(play_remotely_with(connection, board).await)
}

pub async fn play_remotely() {
    let mut connection = None;
    loop {
        loop {
            let connection_f = play_remotely_with_any(connection);
            connection = connection_f.await;
            if connection.is_none()
                || connection.as_ref().unwrap().status.get() != ConnectionStatus::Open
            {
                break;
            }
        }
        if let Some(connection) = connection.as_ref() {
            let play_again = dialoguer::Confirm::new()
                .with_prompt("Do you want to play again?")
                .interact()
                .unwrap();
            if !play_again {
                break;
            }

            connection.status.set(ConnectionStatus::Uninitialized);
        } else {
            break;
        }
    }
}
