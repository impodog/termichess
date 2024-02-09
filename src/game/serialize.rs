use super::*;

impl Piece {
    pub fn serialize(&self) -> String {
        let kind = match self.kind {
            PieceKind::Empty => {
                return "-".to_string();
            }
            PieceKind::King => 'K',
            PieceKind::Rook => 'R',
            PieceKind::Queen => 'Q',
            PieceKind::Bishop => 'B',
            PieceKind::Knight => 'N',
            PieceKind::Pawn => 'P',
        };

        let color = match self.color {
            PieceColor::White => 'w',
            PieceColor::Black => 'b',
        };

        format!("{}{}{}", kind, color, self.previous)
    }

    pub fn deserialize(s: String) -> Option<Self> {
        if s.starts_with('-') {
            return Some(Piece::empty());
        }
        if s.len() < 3 {
            None
        } else {
            let mut chars = s.chars();
            let kind = chars.next().unwrap();
            let color = chars.next().unwrap();
            let previous: usize = chars.collect::<String>().parse().unwrap();

            let kind = match kind {
                'P' => PieceKind::Pawn,
                'K' => PieceKind::King,
                'Q' => PieceKind::Queen,
                'B' => PieceKind::Bishop,
                'N' => PieceKind::Knight,
                'R' => PieceKind::Rook,
                _ => return None,
            };
            let color = match color {
                'w' => PieceColor::White,
                'b' => PieceColor::Black,
                _ => return None,
            };
            Some(Piece {
                kind,
                color,
                previous,
            })
        }
    }
}

impl Move {
    pub fn serialize(&self) -> String {
        format!("{}{}", self.from.to_notation(), self.to.to_notation())
    }

    pub fn deserialize(s: String) -> Option<Self> {
        if s.len() != 4 {
            None
        } else {
            Some(Self {
                from: Square::from_notation(&s[0..=1])?,
                to: Square::from_notation(&s[2..=3])?,
                promotion: None,
            })
        }
    }
}

impl Status {
    pub fn serialize(&self) -> String {
        match *self {
            Status::Playing => "P".to_string(),
            Status::White => "W".to_string(),
            Status::Black => "B".to_string(),
            Status::Draw => "D".to_string(),
        }
    }

    pub fn deserialize(s: String) -> Option<Self> {
        if s.len() != 1 {
            None
        } else {
            Some(match s.chars().next().unwrap() {
                'P' => Status::Playing,
                'W' => Status::White,
                'B' => Status::Black,
                'D' => Status::Draw,
                _ => return None,
            })
        }
    }
}

impl Board {
    pub fn serialize(&self) -> String {
        let squares = self
            .squares
            .iter()
            .map(|rank| {
                rank.iter()
                    .map(|piece| piece.serialize())
                    .collect::<String>()
            })
            .collect::<String>();
        let moves = self
            .moves
            .iter()
            .map(|mv| mv.serialize())
            .collect::<String>();
        let status = self.status.serialize();

        format!(
            "{}/{}/{}/{}/{}",
            squares,
            moves,
            self.turn,
            status,
            if self.draw_offer { 't' } else { 'f' }
        )
    }

    pub fn deserialize(s: String) -> Option<Self> {
        let mut board = Self::new();

        let parts: Vec<String> = s.split('/').map(|v| v.to_string()).collect();
        let mut iter = parts.iter();
        if parts.len() != 5 {
            return None;
        }
        let squares = iter.next().unwrap();
        let moves = iter.next().unwrap();
        board.turn = iter.next().unwrap().parse().map(Some).unwrap_or(None)?;
        let status = iter.next().unwrap();
        board.draw_offer = iter.next().unwrap().chars().next()? == 't';

        let mut file = 0;
        let mut rank = 0;
        let mut iter = squares.chars().peekable();
        while file != 8 {
            let mut str = String::new();
            match iter.next()? {
                '-' => str.push('-'),
                c => {
                    str.push(c);
                    while let Some(c) = iter.peek() {
                        if c.is_uppercase() || *c == '-' {
                            break;
                        }
                        str.push(*c);
                        iter.next()?;
                    }
                }
            }

            board.squares[file][rank] = Piece::deserialize(str)?;

            rank += 1;
            if rank == 8 {
                file += 1;
                rank = 0;
            }
        }

        if moves.len() % 4 != 0 {
            return None;
        }
        let count = moves.len() / 4;

        for i in 0..count {
            board
                .moves
                .push(Move::deserialize(moves[i * 4..(i + 1) * 4].to_string())?);
        }

        board.status = Status::deserialize(status.clone())?;

        board.update();
        board.update_mate();

        Some(board)
    }
}
