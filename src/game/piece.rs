use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceKind {
    Empty,
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
    // This is the turn number when the piece was most recently moved
    pub previous: usize,
}

impl PieceKind {
    pub fn is_empty(&self) -> bool {
        matches!(self, PieceKind::Empty)
    }

    pub fn to_notation(&self) -> Option<&'static str> {
        match self {
            PieceKind::Empty => None,
            PieceKind::King => Some(&"K"),
            PieceKind::Queen => Some(&"Q"),
            PieceKind::Bishop => Some(&"B"),
            PieceKind::Knight => Some(&"N"),
            PieceKind::Rook => Some(&"R"),
            PieceKind::Pawn => Some(&""),
        }
    }

    pub fn from_notation(notation: &str) -> Option<Self> {
        match notation {
            "K" => Some(PieceKind::King),
            "Q" => Some(PieceKind::Queen),
            "B" => Some(PieceKind::Bishop),
            "N" => Some(PieceKind::Knight),
            "R" => Some(PieceKind::Rook),
            "" | "P" => Some(PieceKind::Pawn),
            _ => None,
        }
    }
}

impl PieceColor {
    pub fn opposite(&self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            PieceColor::White => "White",
            PieceColor::Black => "Black",
        }
    }
}

impl From<PieceColor> for console::Color {
    fn from(color: PieceColor) -> Self {
        match color {
            PieceColor::White => console::Color::White,
            PieceColor::Black => console::Color::Black,
        }
    }
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor) -> Self {
        Self {
            kind,
            color,
            previous: 0,
        }
    }

    pub fn empty() -> Self {
        Self {
            kind: PieceKind::Empty,
            color: PieceColor::White,
            previous: 0,
        }
    }

    pub fn unicode(&self) -> &'static str {
        if CONFIG.unicode {
            match self.color {
                PieceColor::White => match self.kind {
                    PieceKind::King => "♔",
                    PieceKind::Queen => "♕",
                    PieceKind::Bishop => "♗",
                    PieceKind::Knight => "♘",
                    PieceKind::Rook => "♖",
                    PieceKind::Pawn => "♙",
                    PieceKind::Empty => "-",
                },
                PieceColor::Black => match self.kind {
                    PieceKind::King => "♚",
                    PieceKind::Queen => "♛",
                    PieceKind::Bishop => "♝",
                    PieceKind::Knight => "♞",
                    PieceKind::Rook => "♜",
                    PieceKind::Pawn => "♟",
                    PieceKind::Empty => "-",
                },
            }
        } else {
            match self.color {
                PieceColor::White => match self.kind {
                    PieceKind::King => "K",
                    PieceKind::Queen => "Q",
                    PieceKind::Bishop => "B",
                    PieceKind::Knight => "N",
                    PieceKind::Rook => "R",
                    PieceKind::Pawn => "P",
                    PieceKind::Empty => "-",
                },
                PieceColor::Black => match self.kind {
                    PieceKind::King => "K",
                    PieceKind::Queen => "Q",
                    PieceKind::Bishop => "B",
                    PieceKind::Knight => "N",
                    PieceKind::Rook => "R",
                    PieceKind::Pawn => "P",
                    PieceKind::Empty => "-",
                },
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.kind.is_empty()
    }

    pub fn is_color(&self, color: PieceColor) -> bool {
        self.color == color
    }

    pub fn is_kind(&self, kind: PieceKind) -> bool {
        self.kind == kind
    }

    pub fn is_friend(&self, color: PieceColor) -> bool {
        !self.is_empty() && self.is_color(color)
    }

    pub fn is_enemy(&self, color: PieceColor) -> bool {
        !self.is_empty() && !self.is_color(color)
    }

    pub fn is_replaceable(&self, color: PieceColor) -> bool {
        self.is_empty() || !self.is_color(color)
    }
}
