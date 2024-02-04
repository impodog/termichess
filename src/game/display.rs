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
        let unicode = self.unicode();
        let is_dark_tile = f.alternate();

        let str = if CONFIG.unicode {
            let mut str = String::from(' ');
            str.extend(unicode.chars());
            str.push(' ');
            str
        } else {
            let c = if self.is_empty() {
                '-'
            } else {
                match self.color {
                    PieceColor::White => ':',
                    PieceColor::Black => '*',
                }
            };
            let mut str = String::from(c);
            str.extend(unicode.chars());
            str.push(c);
            str
        };

        let style = style(str).bold();
        let style = if is_dark_tile {
            style.bg(console::Color::Blue)
        } else {
            style.bg(console::Color::Cyan)
        };

        write!(f, "{}", style)?;
        Ok(())
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let wins = style("Wins").bold().green();
        let draw = style("Draw").bold().yellow();
        match self {
            Status::Playing => write!(f, "Playing"),
            Status::White => write!(f, "{} {}", PieceColor::White, wins),
            Status::Black => write!(f, "{} {}", PieceColor::Black, wins),
            Status::Draw => write!(f, "{}", draw),
        }
    }
}

impl Board {
    fn show_layout(&self, f: &mut Formatter) -> fmt::Result {
        match self.which_color() {
            PieceColor::White => {
                for rank in (0..8).rev() {
                    write!(f, "{} ", rank + 1)?;
                    for file in 0..8 {
                        let square = Square::new(file, rank).unwrap();
                        let piece = self.get(square);

                        let is_dark_tile = (file + rank) % 2 == 1;
                        if is_dark_tile {
                            write!(f, "{:#}", piece)?;
                        } else {
                            write!(f, "{}", piece)?;
                        }
                    }
                    match rank {
                        0 => write!(f, " > White's Turn {}", self.turn / 2 + 1)?,
                        7 => write!(f, " - Black")?,
                        _ => write!(f, " |")?,
                    }
                    writeln!(f)?;
                }
                writeln!(f, "   a  b  c  d  e  f  g  h")?;
            }
            PieceColor::Black => {
                for rank in 0..8 {
                    write!(f, "{} ", rank + 1)?;
                    for file in (0..8).rev() {
                        let square = Square::new(file, rank).unwrap();
                        let piece = self.get(square);

                        let is_dark_tile = (file + rank) % 2 == 1;
                        if is_dark_tile {
                            write!(f, "{:#}", piece)?;
                        } else {
                            write!(f, "{}", piece)?;
                        }
                    }
                    match rank {
                        0 => write!(f, " - White")?,
                        7 => write!(f, " > Black's Turn {}", self.turn / 2)?,
                        _ => write!(f, " |")?,
                    }
                    writeln!(f)?;
                }
                writeln!(f, "   h  g  f  e  d  c  b  a")?;
            }
        }
        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.status == Status::Playing {
            self.show_layout(f)?;

            if self.draw_offer {
                writeln!(f, "{}", style("Your opponent offered you a draw.").yellow())?;
            } else {
                if self.check {
                    writeln!(f, "{}", style("CHECK!").red())?;
                }
            }
        } else {
            self.show_layout(f)?;

            if self.checkmate {
                writeln!(f, "{}", style("CHECKMATE!").red())?;
            }
            writeln!(f, "Game has ended! Result: {}", self.status)?;
        }

        Ok(())
    }
}
