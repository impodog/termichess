#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command {
    Chess(String),
    Resign,
    Draw,
    Chat(String),
}

pub fn parse_raw(str: String) -> Command {
    match str.as_str() {
        "resign" | "quit" | "exit" => Command::Resign,
        "draw" => Command::Draw,
        _ => {
            if str.starts_with("chat") && str.len() >= 5 {
                Command::Chat(str[4..].trim().to_string())
            } else {
                Command::Chess(
                    str.chars()
                        .filter(|c| matches!(c, 'a'..='h' | '0'..='8' | 'Q' | 'R' | 'B' | 'N' | 'K' | 'x' | '='))
                        .collect::<String>(),
                )
            }
        }
    }
}
