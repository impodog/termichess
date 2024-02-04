use super::*;

impl Display for Square {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (b'a' + self.file() as u8) as char,
            self.rank() + 1
        )
    }
}

impl Display for PieceColor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            style(self.name())
                .bg(self.clone().into())
                .fg(self.opposite().into())
        )
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let unicode = self.kind.to_unicode();
        let style = if self.is_empty() {
            style(unicode)
                .bg(console::Color::Blue)
                .fg(console::Color::Black)
        } else {
            style(unicode)
                .bg(self.color.into())
                .fg(self.color.opposite().into())
        };
        write!(f, "{}", style)?;
        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{}", rank + 1)?;
            for file in 0..8 {
                let square = Square::new(file, rank).unwrap();
                let piece = self.get(square);
                write!(f, " {}", piece)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "  a b c d e f g h")?;
        writeln!(f, "Turn Number: {}", self.turn)?;
        writeln!(f, "{} to move!", self.which_color())?;
        if self.check {
            writeln!(f, "{}", style("CHECK!").red())?;
        }
        Ok(())
    }
}
