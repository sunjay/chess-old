use std::fmt;

use owo_colors::{OwoColorize, AnsiColors};

use crate::{ChessTiles, Color};

const VBAR: char = '│';
const HBAR: char = '─';
const CORNER: char = '┼';

/// Wrapper around the board tiles that prints out a colored board
#[derive(Debug, Clone, PartialEq)]
pub struct UnicodeBoard<'a> {
    pub tiles: &'a ChessTiles,
}

impl<'a> UnicodeBoard<'a> {
    pub fn new(tiles: &'a ChessTiles) -> Self {
        Self {
            tiles,
        }
    }
}

impl<'a> fmt::Display for UnicodeBoard<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row_size = self.tiles[0].len();
        let horizontal_line = |f: &mut fmt::Formatter| {
            write!(f, "   ")?;
            for _ in 0..row_size {
                write!(f, "{}{}{}{}", CORNER, HBAR, HBAR, HBAR)?;
            }
            writeln!(f, "{}", CORNER)
        };

        let letters_row = |f: &mut fmt::Formatter| {
            write!(f, "   ")?;
            for i in 0..row_size {
                write!(f, "  {} ", (b'a' + i as u8) as char)?;
            }
            writeln!(f)
        };

        letters_row(f)?;
        horizontal_line(f)?;

        for (i, row) in self.tiles.iter().rev().enumerate() {
            let rank = self.tiles.len() - i;

            write!(f, " {} {}", rank, VBAR)?;

            for tile in row.iter() {
                match tile {
                    &Some(piece) => write!(f, " {} ", piece.type_().color(match piece.color() {
                        Color::White => AnsiColors::White,
                        Color::Black => AnsiColors::BrightMagenta,
                    }))?,
                    None => write!(f, " {} ", ' ')?,
                }

                write!(f, "{}", VBAR)?;
            }
            writeln!(f, " {}", rank)?;

            horizontal_line(f)?;
        }

        letters_row(f)?;

        Ok(())
    }
}
