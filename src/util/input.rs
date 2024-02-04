use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command {
    Chess(String),
    Resign,
    Draw,
}

pub fn input_command() -> Command {
    let str = dialoguer::Input::<String>::new()
        .with_prompt("Command")
        .interact_text()
        .unwrap();
    match str.as_str() {
        "resign" => Command::Resign,
        "draw" => Command::Draw,
        _ => Command::Chess(
            str.chars()
                .filter(|c| match c {
                    'a'..='h' | '0'..='8' | 'Q' | 'R' | 'B' | 'N' | 'K' | 'x' | '=' => true,
                    _ => false,
                })
                .collect::<String>(),
        ),
    }
}
