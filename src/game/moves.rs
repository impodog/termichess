use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub promotion: Option<PieceKind>,
}

impl Move {
    pub fn new(from: Square, to: Square, promotion: Option<PieceKind>) -> Self {
        Move {
            from,
            to,
            promotion,
        }
    }

    pub fn new_normal(from: Square, to: Square) -> Self {
        Move::new(from, to, None)
    }

    pub fn new_promotion(from: Square, to: Square, promotion: PieceKind) -> Option<Self> {
        match promotion {
            PieceKind::Queen | PieceKind::Rook | PieceKind::Bishop | PieceKind::Knight => {
                Some(Move::new(from, to, Some(promotion)))
            }
            _ => None,
        }
    }
}

impl Board {
    pub fn perform(&self, mv: Move) -> Option<Self> {
        let mut board = self.clone();

        let piece = board.get(mv.from);
        let target = board.get(mv.to);

        if piece.is_kind(PieceKind::King) {
            let distance = mv.to.file() as isize - mv.from.file() as isize;
            if distance == -2 {
                board.force(mv.from, mv.to);
                board.force(
                    Square::new(0, mv.from.rank()).unwrap(),
                    Square::new(3, mv.from.rank()).unwrap(),
                );
            } else if distance == 2 {
                board.force(mv.from, mv.to);
                board.force(
                    Square::new(7, mv.from.rank()).unwrap(),
                    Square::new(5, mv.from.rank()).unwrap(),
                );
            } else {
                board.force(mv.from, mv.to);
            }
        } else if piece.is_kind(PieceKind::Pawn) {
            if mv.to.rank() == 0 || mv.to.rank() == 7 {
                board.set(mv.to, Piece::new(mv.promotion.unwrap(), piece.color));
                board.set(mv.from, Piece::empty());
            } else if mv.to.file() != mv.from.file() && target.is_empty() {
                board.set(
                    Square::new(mv.to.file(), mv.from.rank()).unwrap(),
                    Piece::empty(),
                );
                board.force(mv.from, mv.to);
            } else {
                board.force(mv.from, mv.to);
            }
        } else {
            board.force(mv.from, mv.to);
        }

        board.update();
        if board.is_check() {
            None
        } else {
            board.moves.push(mv);

            board.turn += 1;
            board.update();
            board.update_mate();

            Some(board)
        }
    }
}
