use std::fmt;

use crate::{Piece, Color, UnicodeBoard, Coord, File, constants::*};

/// The size of the rows and columns of the chess board
const SIZE: usize = 8;

/// The rows of tiles on a chess board
///
/// The index [0][0] coresponds to the a1 coordinate. It is assumed that white
/// pawns will move `Direction::N`.
pub type ChessTiles = [[Option<Piece>; SIZE]; SIZE];

/// The default board setup with white pieces starting at A1
pub const DEFAULT_BOARD: ChessTiles = [
    [Some(W_R), Some(W_N), Some(W_B), Some(W_Q), Some(W_K), Some(W_B), Some(W_N), Some(W_R)],
    [Some(W_P), Some(W_P), Some(W_P), Some(W_P), Some(W_P), Some(W_P), Some(W_P), Some(W_P)],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [Some(B_P), Some(B_P), Some(B_P), Some(B_P), Some(B_P), Some(B_P), Some(B_P), Some(B_P)],
    [Some(B_R), Some(B_N), Some(B_B), Some(B_Q), Some(B_K), Some(B_B), Some(B_N), Some(B_R)],
];

/// The piece that a pawn is promoted to upon reaching the other side of the board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PawnPromotion {
    Queen,
    Bishop,
    Knight,
    Rook,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RookFile {
    A,
    H,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Move {
    /// Move the piece at `from` to the position `to`. The piece must be the color of the current
    /// player and the move must be valid for the piece's type.
    Move {from: Coord, to: Coord},

    /// Promote the pawn on the second to last rank of the given file to the given piece
    ///
    /// The pawn must be on the 7th rank for white and the 2nd rank for black (depending on the
    /// current turn).
    Promote {pawn: File, promote_to: PawnPromotion},

    /// Moves the king two tiles towards the given rook and the rook to the other side of the king.
    Castle {rook: RookFile},
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChessBoard {
    tiles: ChessTiles,
    /// The player who will make the next move
    current_turn: Color,
    /// If true, the current player's king is being attacked by at least one of the opponent's pieces
    is_in_check: bool,
}

impl Default for ChessBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl ChessBoard {
    /// Creates a new chess board with the default arrangement of pieces
    pub const fn new() -> Self {
        Self {
            tiles: DEFAULT_BOARD,
            current_turn: Color::White,
            is_in_check: false,
        }
    }

    /// Returns the player that should make the next move
    pub const fn current_turn(&self) -> Color {
        self.current_turn
    }

    /// Transforms the board into a 2D array of pieces
    pub fn to_tiles(&self) -> ChessTiles {
        self.tiles
    }

    pub fn display(&self) -> impl fmt::Display + '_ {
        UnicodeBoard::new(&self.tiles)
    }

    pub fn make_move(&self, pmove: Move) {
        todo!()
    }
}
