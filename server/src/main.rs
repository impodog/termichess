use ::server::*;
pub use async_attributes::{main, test};
use async_std::sync::{Arc, RwLock};
use std::collections::HashMap;
use tide::log::*;
use tide::{prelude::*, Error};
use tide::{Request, StatusCode};

#[derive(Debug, Clone)]
struct RoomInfo {
    turn: bool,
    joined: usize,
    queue: Option<String>,
}

type RoomMap = HashMap<RoomCode, RoomInfo>;

#[main]
async fn main() -> tide::Result<()> {
    with_level(LevelFilter::Debug);

    let mut app = tide::new();
    let rooms = Arc::new(RwLock::new(RoomMap::new()));
    {
        let rooms_ = rooms.clone();
        app.at("/chess/login")
            .post(move |req| chess_login(req, rooms_.clone()));

        let rooms_ = rooms.clone();
        app.at("/chess/play")
            .post(move |req| chess_play(req, rooms_.clone()));

        let rooms_ = rooms.clone();
        app.at("/chess/query")
            .post(move |req| chess_query(req, rooms_.clone()));

        let rooms_ = rooms.clone();
        app.at("/chess/logout")
            .post(move |req| chess_logout(req, rooms_.clone()));

        let rooms_ = rooms.clone();
        app.at("/chess/is_ok")
            .post(move |req| chess_is_ok(req, rooms_.clone()));
    }

    info!("Starting server on http://127.0.0.1:8080");

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn chess_login(mut req: Request<()>, map: Arc<RwLock<RoomMap>>) -> tide::Result {
    let login: LoginRequest = req.body_json().await?;
    {
        let mut map = map.write().await;

        if map.contains_key(&login.room) {
            let info = map.get_mut(&login.room).unwrap();
            match info.joined {
                0 => {
                    info.joined += 1;
                    info!("Player joined the room: {:?}", login.room);
                    Ok(json!(LoginResponse { player: true }).into())
                }
                1 => {
                    info.joined += 1;
                    info!("Player joined the room: {:?}", login.room);
                    Ok(json!(LoginResponse { player: false }).into())
                }
                _ => {
                    warn!("Room is full: {:?}", login.room);
                    Err(Error::new(StatusCode::Conflict, ServerError))
                }
            }
        } else {
            map.insert(
                login.room,
                RoomInfo {
                    turn: true,
                    joined: 1,
                    queue: None,
                },
            );
            info!("New room created: {:?}", login.room);
            Ok(json!(LoginResponse { player: true }).into())
        }
    }
}

async fn chess_play(mut req: Request<()>, map: Arc<RwLock<RoomMap>>) -> tide::Result {
    let command: CommandRequest = req.body_json().await?;
    {
        let mut map = map.write().await;

        if map.contains_key(&command.room) {
            let info = map.get_mut(&command.room).unwrap();
            if info.joined == 2 {
                if info.turn == command.player && info.queue.is_none() {
                    info.turn = !info.turn;
                    info.queue = Some(command.cmd);
                    info!("Player played a move in room: {:?}", command.room);
                    Ok(json!({}).into())
                } else {
                    info!(
                        "It's not your turn when playing in room: {:?}",
                        command.room
                    );
                    Err(Error::new(StatusCode::Locked, ServerError))
                }
            } else {
                warn!("Not enough players in room: {:?}", command.room);
                Err(Error::new(StatusCode::NotAcceptable, ServerError))
            }
        } else {
            warn!("Room not found: {:?}", command.room);
            Err(Error::new(StatusCode::NotFound, ServerError))
        }
    }
}

async fn chess_query(mut req: Request<()>, map: Arc<RwLock<RoomMap>>) -> tide::Result {
    let query: QueryRequest = req.body_json().await?;
    {
        let mut map = map.write().await;

        if map.contains_key(&query.room) {
            let info = map.get_mut(&query.room).unwrap();
            if info.joined == 2 {
                if info.turn == query.player {
                    if let Some(cmd) = info.queue.clone() {
                        info.queue = None;
                        info!("Player queried the room: {:?}", query.room);
                        Ok(json!(QueryResponse { cmd }).into())
                    } else {
                        warn!("No move available in room: {:?}", query.room);
                        Err(Error::new(StatusCode::NotFound, ServerError))
                    }
                } else {
                    warn!("It's not your turn when querying in room: {:?}", query.room);
                    Err(Error::new(StatusCode::Locked, ServerError))
                }
            } else {
                warn!("Not enough players in room: {:?}", query.room);
                Err(Error::new(StatusCode::NotAcceptable, ServerError))
            }
        } else {
            println!("Room not found: {:?}", query.room);
            Err(Error::new(StatusCode::NotFound, ServerError))
        }
    }
}

async fn chess_logout(mut req: Request<()>, map: Arc<RwLock<RoomMap>>) -> tide::Result {
    let logout: LogoutRequest = req.body_json().await?;
    {
        let mut map = map.write().await;

        if map.contains_key(&logout.room) {
            map.remove(&logout.room);
            info!("Player logged out of room: {:?}", logout.room);
            Ok(json!({}).into())
        } else {
            warn!("Room not found: {:?}", logout.room);
            Err(Error::new(StatusCode::NotFound, ServerError))
        }
    }
}

async fn chess_is_ok(mut req: Request<()>, map: Arc<RwLock<RoomMap>>) -> tide::Result {
    let is_ok: IsOkRequest = req.body_json().await?;
    {
        let map = map.read().await;

        let ok = map.contains_key(&is_ok.room) && map.get(&is_ok.room).unwrap().joined == 2;
        info!("Is ok value : {:?}", ok);
        Ok(json!(IsOkResponse { ok }).into())
    }
}
