use crate::{Piece, Color, constants::*};

/// The size of the rows and columns of the chess board
const SIZE: usize = 8;

/// The rows of tiles on a chess board
///
/// The index [0][0] coresponds to the A1 coordinate. It is assumed that white
/// pawns will move `Direction::N`.
pub type ChessTiles = [[Option<Piece>; SIZE]; SIZE];

/// The default board setup with white at the bottom
pub const DEFAULT_BOARD: ChessTiles = [
    [Some(B_R), Some(B_N), Some(B_B), Some(B_K), Some(B_Q), Some(B_B), Some(B_N), Some(B_R)],
    [Some(B_P), Some(B_P), Some(B_P), Some(B_P), Some(B_P), Some(B_P), Some(B_P), Some(B_P)],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [None, None, None, None, None, None, None, None],
    [Some(W_P), Some(W_P), Some(W_P), Some(W_P), Some(W_P), Some(W_P), Some(W_P), Some(W_P)],
    [Some(W_R), Some(W_N), Some(W_B), Some(W_K), Some(W_Q), Some(W_B), Some(W_N), Some(W_R)],
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChessBoard {
    tiles: ChessTiles,
    /// The player who will make the next move
    current_turn: Color,
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
}
