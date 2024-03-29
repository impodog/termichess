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
                .bg((*self).into())
                .fg(self.opposite().into())
        )
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let unicode = self.unicode();
        let is_dark_tile = f.alternate();
        let is_used_tile = f.sign_plus();

        let str = if CONFIG.unicode {
            let mut str = String::new();
            for _ in 0..CONFIG.spacing.div_ceil(2) - 1 {
                str.push(' ');
            }
            str.push_str(unicode);
            for i in 0..CONFIG.spacing.div_euclid(2) {
                if i == 0 && CONFIG.spacing % 2 == 0 && self.is_empty() {
                    str.push_str(unicode);
                } else {
                    str.push(' ');
                }
            }
            str
        } else {
            let c = if self.is_empty() {
                ' '
            } else {
                match self.color {
                    PieceColor::White => ':',
                    PieceColor::Black => '*',
                }
            };
            let mut str = String::from(c);
            str.push_str(unicode);
            str.push(c);
            str
        };

        let style = style(str).bold().fg(console::Color::Black);
        let style = if is_used_tile {
            style.bg(console::Color::Red)
        } else {
            if is_dark_tile {
                style.bg(console::Color::White)
            } else {
                style.bg(console::Color::Magenta)
            }
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
    fn show_rank_flip(&self, f: &mut Formatter, rank: usize, flip: bool) -> fmt::Result {
        if flip {
            match rank {
                7 => write!(f, " > Black's Turn {}", self.turn / 2)?,
                0 => write!(f, " - White")?,
                _ => write!(f, " |")?,
            }
        } else {
            match rank {
                7 => write!(f, " - Black")?,
                0 => write!(f, " > White's Turn {}", self.turn / 2 + 1)?,
                _ => write!(f, " |")?,
            }
        }
        Ok(())
    }
    fn show_layout(&self, f: &mut Formatter, flip: bool) -> fmt::Result {
        if (self.which_color() == PieceColor::White) ^ flip {
            for rank in (0..8).rev() {
                write!(f, "{} ", rank + 1)?;
                for file in 0..8 {
                    let square = Square::new(file, rank).unwrap();
                    let piece = self.get(square);

                    let is_used_tile = self
                        .moves
                        .last()
                        .map_or(false, |m| m.from == square || m.to == square);
                    let is_dark_tile = (file + rank) % 2 == 1;
                    if is_used_tile {
                        write!(f, "{:+}", piece)?;
                    } else {
                        if is_dark_tile {
                            write!(f, "{:#}", piece)?;
                        } else {
                            write!(f, "{}", piece)?;
                        }
                    }
                }
                self.show_rank_flip(f, rank, flip)?;
                writeln!(f)?;
            }
            write!(f, " ")?;

            let spacing = CONFIG.get_spaces();
            for i in 0..8 {
                let c = (b'a' + i) as char;
                write!(
                    f,
                    "{}{}{}",
                    spacing,
                    if CONFIG.spacing % 2 == 0 { c } else { ' ' },
                    c
                )?;
            }
            writeln!(f)?;
        } else {
            for rank in 0..8 {
                write!(f, "{} ", rank + 1)?;
                for file in (0..8).rev() {
                    let square = Square::new(file, rank).unwrap();
                    let piece = self.get(square);

                    let is_used_tile = self
                        .moves
                        .last()
                        .map_or(false, |m| m.from == square || m.to == square);
                    let is_dark_tile = (file + rank) % 2 == 1;
                    if is_used_tile {
                        write!(f, "{:+}", piece)?;
                    } else {
                        if is_dark_tile {
                            write!(f, "{:#}", piece)?;
                        } else {
                            write!(f, "{}", piece)?;
                        }
                    }
                }
                self.show_rank_flip(f, rank, !flip)?;
                writeln!(f)?;
            }
            write!(f, " ")?;

            let spacing = CONFIG.get_spaces();
            for i in 0..8 {
                let c = (b'h' - i) as char;
                write!(
                    f,
                    "{}{}{}",
                    spacing,
                    if CONFIG.spacing % 2 == 0 { c } else { ' ' },
                    c
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let flip = f.alternate();
        if self.status == Status::Playing {
            if self.draw_offer {
                if flip {
                    writeln!(f, "{}", style("You offered opponent a draw.").yellow())?;
                } else {
                    writeln!(f, "{}", style("Opponent offered you a draw.").yellow())?;
                }
            } else {
                self.show_layout(f, flip)?;
                if self.check {
                    writeln!(f, "{}", style("CHECK!").red())?;
                }
            }
        } else {
            self.show_layout(f, flip)?;

            if self.no_safe {
                if self.check {
                    writeln!(f, "{}", style("CHECKMATE!").red())?;
                } else {
                    writeln!(f, "{}", style("STALEMATE!").yellow())?;
                }
            }
            writeln!(f, "Game has ended! Result: {}", self.status)?;
        }

        Ok(())
    }
}
