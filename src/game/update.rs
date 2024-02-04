use super::*;

impl Board {
    pub fn update(&mut self) {
        let color = self.which_color();
        let opposite = color.opposite();

        self.reachable = Default::default();
        self.threatened = Default::default();

        self.check = false;

        for file in 0..8 {
            for rank in 0..8 {
                let square = Square::new(file, rank).unwrap();
                let piece = self.get(square);
                if !piece.is_empty() && piece.is_color(opposite) {
                    self.reachable[file][rank] = self.reachable(square);
                    self.threatened.extend(
                        self.reachable[file][rank]
                            .iter()
                            .map(|square| square.clone()),
                    );
                }
            }
        }
        for file in 0..8 {
            for rank in 0..8 {
                let square = Square::new(file, rank).unwrap();
                let piece = self.get(square);
                if !piece.is_empty() && piece.is_color(color) {
                    self.reachable[file][rank] = self.reachable(square);

                    if piece.kind == PieceKind::King && self.is_threatened(square) {
                        self.check = true;
                    }
                }
            }
        }
    }
}
