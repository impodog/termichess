use super::*;

impl Board {
    pub fn update(&mut self) {
        let color = self.which_color();

        self.reachable = Default::default();
        self.threatened = Default::default();

        self.check = false;

        for file in 0..8 {
            for rank in 0..8 {
                let square = Square::new(file, rank).unwrap();
                let piece = self.get(square);
                if !piece.is_enemy(color) {
                    continue;
                }
                self.reachable[file][rank] = self.reachable(square);
                self.threatened
                    .extend(self.reachable[file][rank].iter().copied());
            }
        }
        for file in 0..8 {
            for rank in 0..8 {
                let square = Square::new(file, rank).unwrap();
                let piece = self.get(square);
                if piece.is_friend(color) {
                    self.reachable[file][rank] = self.reachable(square);

                    if piece.kind == PieceKind::King && self.is_threatened(square) {
                        self.check = true;
                    }
                }
            }
        }
    }

    fn check_no_safe(&self) -> bool {
        let color = self.which_color();
        let is_safe = Arc::new(RwLock::new(false));

        let mut king = None;
        for file in 0..8 {
            for rank in 0..8 {
                let square = Square::new(file, rank).unwrap();
                let piece = self.get(square);
                if !piece.is_empty() && piece.is_color(color) && piece.is_kind(PieceKind::King) {
                    king = Some(square);
                    break;
                }
            }
        }
        let king = king.unwrap();

        for file in 0..8 {
            for rank in 0..8 {
                let square = Square::new(file, rank).unwrap();
                let reachable = self.reachable[square.file()][square.rank()].clone();

                let piece = self.get(square);
                if !piece.is_friend(color) {
                    continue;
                }

                let is_safe = is_safe.clone();
                let board = self.clone();

                let func = move || {
                    for target in reachable {
                        let mut board = board.clone();
                        board.force(square, target);
                        board.update();

                        let king = if square == king { target } else { king };

                        if !board.is_threatened(king) {
                            //println!("SAFE {} -> {}", square, target);
                            *is_safe.write().unwrap() = true;
                            return;
                        }
                        // println!("{}", board);
                        // dialoguer::Confirm::new()
                        //     .with_prompt("Continue")
                        //     .interact()
                        //     .unwrap();
                    }
                };
                self.pool.write().unwrap().execute(func);
            }
        }

        self.pool.read().unwrap().join();

        let is_safe = *is_safe.read().unwrap();
        !is_safe
    }

    pub fn update_mate(&mut self) {
        if self.check_no_safe() {
            self.no_safe = true;
            if self.check {
                self.status = match self.which_color() {
                    PieceColor::White => Status::Black,
                    PieceColor::Black => Status::White,
                };
            } else {
                self.status = Status::Draw;
            }
        }
    }
}
