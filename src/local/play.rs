use super::*;

pub fn play_locally() {
    let mut board = game::Board::new();
    let mut err = None;
    let error = style("Error").red();

    while board.status == game::Status::Playing {
        println!("{}", board);
        if let Some(msg) = err {
            println!("{}: {}", error, msg);
            err = None;
        }
        //board.show_piece_info();
        let str = dialoguer::Input::<String>::new()
            .with_prompt("Command")
            .interact()
            .unwrap();
        let command = util::parse_raw(str);

        if board.draw_offer && command != util::Command::Draw {
            println!("Draw offer declined!");
            board.decline_draw();
            continue;
        }

        match command {
            util::Command::Chess(str) => {
                let notation = board.translate(&str);
                if let Ok(notation) = notation {
                    let next = board.perform(notation);
                    if next.is_none() {
                        err = Some("Invalid move! This leads to a check!".to_string());
                    } else {
                        board = next.unwrap();
                    }
                } else {
                    err = Some(format!("{}", notation.unwrap_err()));
                }
            }
            util::Command::Resign => {
                println!("You resigned!");
                board.resign();
            }
            util::Command::Draw => {
                println!("You offered a draw!");
                board.draw();
            }
        }
        println!();
    }
    println!("{}", board);
}
