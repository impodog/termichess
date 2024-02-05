#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command {
    Chess(String),
    Resign,
    Draw,
}

pub fn parse_raw(str: String) -> Command {
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
