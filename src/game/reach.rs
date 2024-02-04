use super::*;

impl Board {
    pub fn reachable(&self, square: Square) -> HashSet<Square> {
        let piece = self.get(square);

        match piece.kind {
            PieceKind::Empty => HashSet::new(),
            PieceKind::King => self.reachable_king(square, piece),
            PieceKind::Queen => self.reachable_queen(square, piece),
            PieceKind::Bishop => self.reachable_bishop(square, piece),
            PieceKind::Knight => self.reachable_knight(square, piece),
            PieceKind::Rook => self.reachable_rook(square, piece),
            PieceKind::Pawn => self.reachable_pawn(square, piece),
        }
    }

    fn reachable_king(&self, square: Square, piece: Piece) -> HashSet<Square> {
        let mut squares = HashSet::new();

        for file in -1..=1 {
            for rank in -1..=1 {
                if file == 0 && rank == 0 {
                    continue;
                }

                if let Some(target) = square.offset(file, rank) {
                    if self.get(target).color != piece.color {
                        squares.insert(target);
                    }
                }
            }
        }

        if piece.previous == 0 && piece.color == self.which_color() {
            if let Some(_rook) = self.get_kingside_rook() {
                let mut ok = true;

                for disp in 0..=3 {
                    if let Some(square) = square.offset(disp, 0) {
                        if self.is_threatened(square) {
                            ok = false;
                            break;
                        }
                    }
                }
                for disp in 1..=2 {
                    if let Some(square) = square.offset(disp, 0) {
                        if !self.get(square).is_empty() {
                            ok = false;
                            break;
                        }
                    }
                }

                if ok {
                    squares.insert(Square::new(6, square.rank()).unwrap());
                }
            }

            if let Some(_rook) = self.get_queenside_rook() {
                let mut ok = true;

                for disp in 0..=4 {
                    if let Some(square) = square.offset(-disp, 0) {
                        if self.is_threatened(square) {
                            ok = false;
                            break;
                        }
                    }
                }
                for disp in 1..=3 {
                    if let Some(square) = square.offset(-disp, 0) {
                        if !self.get(square).is_empty() {
                            ok = false;
                            break;
                        }
                    }
                }

                if ok {
                    squares.insert(Square::new(2, square.rank()).unwrap());
                }
            }
        }

        squares
    }

    fn get_kingside_rook(&self) -> Option<Piece> {
        let color = self.which_color();
        let rank = match color {
            PieceColor::White => 0,
            PieceColor::Black => 7,
        };

        let rook = Square::new(7, rank)?;
        let piece = self.get(rook);

        if piece.kind != PieceKind::Rook || piece.previous != 0 {
            return None;
        }

        Some(piece)
    }

    fn get_queenside_rook(&self) -> Option<Piece> {
        let color = self.which_color();
        let rank = match color {
            PieceColor::White => 0,
            PieceColor::Black => 7,
        };

        let rook = Square::new(0, rank)?;
        let piece = self.get(rook);

        if piece.kind != PieceKind::Rook || piece.previous != 0 {
            return None;
        }

        Some(piece)
    }

    fn reachable_queen(&self, square: Square, piece: Piece) -> HashSet<Square> {
        let mut squares = HashSet::new();

        squares.extend(self.reachable_bishop(square, piece));
        squares.extend(self.reachable_rook(square, piece));

        squares
    }

    fn reachable_bishop(&self, square: Square, piece: Piece) -> HashSet<Square> {
        let mut squares = HashSet::new();

        for file in -1..=1 {
            for rank in -1..=1 {
                if file == 0 || rank == 0 {
                    continue;
                }

                for distance in 1..=7 {
                    if let Some(target) = square.offset(file * distance, rank * distance) {
                        if self.get(target).is_friend(piece.color) {
                            break;
                        }
                        squares.insert(target);
                        if self.get(target).is_enemy(piece.color) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        squares
    }

    fn reachable_knight(&self, square: Square, piece: Piece) -> HashSet<Square> {
        let mut squares = HashSet::new();

        let offsets = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];

        for &(file_offset, rank_offset) in &offsets {
            if let Some(target) = square.offset(file_offset, rank_offset) {
                if self.get(target).is_replaceable(piece.color) {
                    squares.insert(target);
                }
            }
        }

        squares
    }

    fn reachable_rook(&self, square: Square, piece: Piece) -> HashSet<Square> {
        let mut squares = HashSet::new();

        for file in -1..=1 {
            for rank in -1..=1 {
                if file != 0 && rank != 0 {
                    continue;
                }

                for distance in 1..=7 {
                    if let Some(target) = square.offset(file * distance, rank * distance) {
                        if self.get(target).is_friend(piece.color) {
                            break;
                        }
                        squares.insert(target);
                        if self.get(target).is_enemy(piece.color) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        squares
    }

    fn reachable_pawn(&self, square: Square, piece: Piece) -> HashSet<Square> {
        let mut squares = HashSet::new();

        let direction = match piece.color {
            PieceColor::White => 1,
            PieceColor::Black => -1,
        };

        if let Some(target) = square.offset(0, direction) {
            if self.get(target).is_empty() {
                squares.insert(target);
            }
        }

        if let Some(target) = square.offset(1, direction) {
            if self.get(target).is_enemy(piece.color) {
                squares.insert(target);
            }
        }

        if let Some(target) = square.offset(-1, direction) {
            if self.get(target).is_enemy(piece.color) {
                squares.insert(target);
            }
        }

        if piece.previous == 0 {
            if let (Some(target), Some(passage)) =
                (square.offset(0, direction * 2), square.offset(0, direction))
            {
                if self.get(target).is_empty() && self.get(passage).is_empty() {
                    squares.insert(target);
                }
            }
        }

        if let Some(target) = self.possible_en_passant() {
            if let Some(pawn) = target.offset(1, -direction) {
                if pawn == square {
                    squares.insert(target);
                }
            }

            if let Some(pawn) = target.offset(-1, -direction) {
                if pawn == square {
                    squares.insert(target);
                }
            }
        }

        squares
    }
}
