use super::*;

macro_rules! err {
    ($($msg:tt)*) => {
        NotationError::new(&format!($($msg)*))
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Notation {
    piece: PieceKind,
    from: Option<Square>,
    take: bool,
    to: Square,
    promotion: Option<PieceKind>,
}

#[derive(Debug, Clone)]
pub struct NotationError {
    pub msg: String,
}

impl NotationError {
    pub fn new(msg: &str) -> Self {
        NotationError {
            msg: msg.to_string(),
        }
    }
}

impl std::fmt::Display for NotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Board {
    pub fn translate(&self, notation: &str) -> Result<Move, NotationError> {
        let notation = self.split_notation(notation)?;

        let color = self.which_color();

        println!("Lexical Notation: {:?}", notation);
        let from = if notation.from.is_none() {
            let mut candidate = None;
            for file in 0..8 {
                for rank in 0..8 {
                    let square = Square::new(file, rank).unwrap();
                    let piece = self.get(square);
                    if !piece.is_empty()
                        && piece.is_color(color)
                        && piece.is_kind(notation.piece)
                        && self.reachable[square.file()][square.rank()].contains(&notation.to)
                    {
                        if candidate.is_none() {
                            candidate = Some(square);
                        } else {
                            return Err(err!("Ambiguous move, please specify the source square"));
                        }
                    }
                }
            }
            candidate.ok_or(err!("No piece can move to the target square"))?
        } else {
            let from = notation.from.unwrap();
            if self.reachable[from.file()][from.rank()].contains(&notation.to) {
                from
            } else {
                return Err(err!(
                    "The explicitly specified piece {} cannot move to the target square",
                    from
                ));
            }
        };
        let to = notation.to;

        if notation.take {
            let piece = self.get(to);
            if piece.is_empty() || piece.is_color(color) {
                return Err(err!("No piece to take, remove 'x' from the notation"));
            }
        } else {
            let piece = self.get(to);
            if !piece.is_empty() {
                return Err(err!(
                    "Target square is not empty, add 'x' in between to take the piece"
                ));
            }
        }

        let piece = self.get(from);

        if let Some(promotion) = notation.promotion {
            if !piece.is_kind(PieceKind::Pawn) {
                return Err(err!("Only pawns can be promoted"));
            }

            if let 0 | 7 = to.rank() {
                Ok(Move::new_promotion(from, to, promotion)
                    .ok_or(err!("Invalid promotion code"))?)
            } else {
                Err(err!(
                    "Pawns can only be promoted on the last rank, remove '=...' from the notation"
                ))
            }
        } else {
            if piece.is_kind(PieceKind::Pawn) && (to.rank() == 0 || to.rank() == 7) {
                return Err(err!(
                    "Pawns must be promoted on the last rank, add '=...' to the notation"
                ));
            }

            Ok(Move::new_normal(from, to))
        }
    }

    fn split_notation(&self, notation: &str) -> Result<Notation, NotationError> {
        match notation {
            "00" => {
                let color = self.which_color();
                let rank = if color == PieceColor::White { 0 } else { 7 };
                Ok(Notation {
                    piece: PieceKind::King,
                    from: Some(Square::new(4, rank).unwrap()),
                    take: false,
                    to: Square::new(6, rank).unwrap(),
                    promotion: None,
                })
            }
            "000" => {
                let color = self.which_color();
                let rank = if color == PieceColor::White { 0 } else { 7 };
                Ok(Notation {
                    piece: PieceKind::King,
                    from: Some(Square::new(4, rank).unwrap()),
                    take: false,
                    to: Square::new(2, rank).unwrap(),
                    promotion: None,
                })
            }
            _ => {
                let chars = notation.chars().collect::<Vec<_>>();
                let mut notation = Notation {
                    piece: PieceKind::Pawn,
                    from: None,
                    take: false,
                    to: Square::new(0, 0).unwrap(),
                    promotion: None,
                };

                let mut cur = 0;

                if chars
                    .get(0)
                    .ok_or(err!("Piece code required"))?
                    .is_uppercase()
                {
                    notation.piece =
                        PieceKind::from_notation(&chars[0..=0].iter().collect::<String>())
                            .ok_or(err!("Invalid piece code {}", chars[0]))?;
                    cur += 1
                } else {
                    notation.piece = PieceKind::Pawn;
                }

                let mut first = None;
                if let 'a'..='h' = *chars
                    .get(cur)
                    .ok_or(err!("At least one square code is required"))?
                {
                    chars.get(cur + 1).ok_or(err!("Incomplete square code"))?;

                    if let Some('1'..='8') = chars.get(cur + 1) {
                        first = Some(
                            Square::from_notation(&chars[cur..=cur + 1].iter().collect::<String>())
                                .ok_or(err!(
                                    "Invalid square code {}",
                                    chars[cur..=cur + 1].iter().collect::<String>()
                                ))?,
                        );
                        cur += 2;
                    } else {
                        first = Some(
                            self.find_piece_shortcut(chars[cur], notation.piece)
                                .ok_or(err!(
                                "Shortcut square cannot be found, it either is ambiguous or does not exist"
                            ))?,
                        );
                        cur += 1;
                    }
                }

                if let Some('x') = chars.get(cur) {
                    notation.take = true;
                    cur += 1;
                }

                let mut second = None;
                if let Some('a'..='h') = chars.get(cur) {
                    chars.get(cur + 1).ok_or(err!("Incomplete square code"))?;

                    if let Some('1'..='8') = chars.get(cur + 1) {
                        second = Some(
                            Square::from_notation(&chars[cur..=cur + 1].iter().collect::<String>())
                                .ok_or(err!(
                                    "Invalid square code {}",
                                    chars[cur..=cur + 1].iter().collect::<String>()
                                ))?,
                        );
                        cur += 2;
                    } else {
                        second = Some(
                            self.find_piece_shortcut(chars[cur], notation.piece)
                                .ok_or(err!(
                                "Shortcut square cannot be found, it either is ambiguous or does not exist"
                            ))?,
                        );
                        cur += 1;
                    }
                }

                if let Some(first) = first {
                    if let Some(second) = second {
                        notation.to = second;
                        notation.from = Some(first);
                    } else {
                        notation.to = first;
                    }
                } else {
                    notation.to = second.ok_or(err!("Target square is not specified"))?;
                }

                if let Some('=') = chars.get(cur) {
                    cur += 1;
                    chars.get(cur).ok_or(err!("Promotion code is missing"))?;
                    notation.promotion = Some(
                        PieceKind::from_notation(&chars[cur..=cur].iter().collect::<String>())
                            .ok_or(err!(
                                "Invalid promotion code {}",
                                chars[cur..cur + 1].iter().collect::<String>()
                            ))?,
                    );
                    cur += 1;
                }

                Ok(notation)
            }
        }
    }
}
