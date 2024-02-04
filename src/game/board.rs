use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square(usize, usize);

#[derive(Debug, Clone)]
pub struct Board {
    pub squares: [[Piece; 8]; 8],
    pub turn: usize,

    pub reachable: [[HashSet<Square>; 8]; 8],
    pub threatened: HashSet<Square>,
    pub moves: Vec<Move>,

    pub check: bool,
}

impl Square {
    pub fn new(file: usize, rank: usize) -> Option<Self> {
        match (file, rank) {
            (0..=7, 0..=7) => Some(Square(file, rank)),
            _ => None,
        }
    }

    pub fn file(&self) -> usize {
        self.0
    }

    pub fn rank(&self) -> usize {
        self.1
    }

    pub fn to_notation(&self) -> String {
        let file = (b'a' + self.file() as u8) as char;
        let rank = (b'1' + self.rank() as u8) as char;
        format!("{}{}", file, rank)
    }

    pub fn from_notation(notation: &str) -> Option<Self> {
        if notation.len() != 2 {
            return None;
        }

        let mut chars = notation.chars();
        match (chars.nth(0)?, chars.nth(0)?) {
            ('a'..='h', '1'..='8') => {
                let mut chars = notation.chars();
                let file = chars.nth(0)? as u8 - 'a' as u8;
                let rank = chars.nth(0)? as u8 - '1' as u8;
                Square::new(file as usize, rank as usize)
            }
            _ => None,
        }
    }

    pub fn offset(&self, file: isize, rank: isize) -> Option<Self> {
        let file = self.file() as isize + file;
        let rank = self.rank() as isize + rank;
        match (file, rank) {
            (0..=7, 0..=7) => Some(Square(file as usize, rank as usize)),
            _ => None,
        }
    }
}

impl Board {
    fn new_empty() -> Self {
        let reachable: [[HashSet<Square>; 8]; 8] = Default::default();

        Board {
            squares: [[Piece::empty(); 8]; 8],
            turn: 1,

            reachable,
            threatened: HashSet::new(),
            moves: Vec::new(),

            check: false,
        }
    }

    pub fn new() -> Self {
        let mut board = Board::new_empty();

        for file in 0..8 {
            board.squares[file][1] = Piece::new(PieceKind::Pawn, PieceColor::White);
            board.squares[file][6] = Piece::new(PieceKind::Pawn, PieceColor::Black);
        }

        let back_rank = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];

        for (file, &kind) in back_rank.iter().enumerate() {
            board.squares[file][0] = Piece::new(kind, PieceColor::White);
            board.squares[file][7] = Piece::new(kind, PieceColor::Black);
        }

        board.update();

        board
    }

    pub fn which_color(&self) -> PieceColor {
        match self.turn % 2 {
            0 => PieceColor::Black,
            _ => PieceColor::White,
        }
    }

    pub fn get(&self, square: Square) -> Piece {
        *self
            .squares
            .get(square.file())
            .unwrap()
            .get(square.rank())
            .unwrap()
    }

    pub fn set(&mut self, square: Square, piece: Piece) {
        self.squares[square.file()][square.rank()] = piece;
    }

    pub fn possible_en_passant(&self) -> Option<Square> {
        let last_move = self.moves.last()?;

        if last_move.to.rank() == 3
            && last_move.from.rank() == 1
            && self.get(last_move.to).kind == PieceKind::Pawn
        {
            Some(Square(last_move.to.file(), 2))
        } else if last_move.to.rank() == 4
            && last_move.from.rank() == 6
            && self.get(last_move.to).kind == PieceKind::Pawn
        {
            Some(Square(last_move.to.file(), 5))
        } else {
            None
        }
    }

    pub fn force(&mut self, from: Square, to: Square) -> Option<()> {
        let mut piece = self.get(from);

        if piece.is_empty() {
            return None;
        }

        piece.previous = self.turn;

        self.set(to, piece);
        self.set(from, Piece::empty());

        Some(())
    }

    pub fn push_move(&mut self, movement: Move) {
        self.moves.push(movement);
    }

    pub fn find_piece_shortcut(&self, short: char, kind: PieceKind) -> Option<Square> {
        match short {
            'a'..='h' => {
                let mut candidate = None;
                for rank in 0..8 {
                    let square = Square::new((short as u8 - 'a' as u8) as usize, rank).unwrap();
                    let piece = self.get(square);
                    if !piece.is_empty()
                        && piece.is_color(self.which_color())
                        && piece.is_kind(kind)
                    {
                        if candidate.is_some() {
                            return None;
                        } else {
                            candidate = Some(square);
                        }
                    }
                }
                candidate
            }
            '1'..='8' => {
                let mut candidate = None;
                for file in 0..8 {
                    let square = Square::new(file, (short as u8 - '1' as u8) as usize).unwrap();
                    let piece = self.get(square);
                    if !piece.is_empty()
                        && piece.is_color(self.which_color())
                        && piece.is_kind(kind)
                    {
                        if candidate.is_some() {
                            return None;
                        } else {
                            candidate = Some(square);
                        }
                    }
                }
                candidate
            }
            _ => None,
        }
    }

    pub fn is_threatened(&self, square: Square) -> bool {
        self.threatened.contains(&square)
    }

    pub fn is_check(&self) -> bool {
        self.check
    }

    pub fn show_piece_info(&self) {
        for file in 0..8 {
            for rank in 0..8 {
                let square = Square::new(file, rank).unwrap();
                let piece = self.get(square);
                print!("{} {}", square, piece);
                print!(" ->");
                for reachable in &self.reachable[file][rank] {
                    print!(" {}", reachable);
                }
                println!()
            }
        }
    }
}
