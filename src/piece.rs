use std::fmt;

pub mod constants {
    use super::Piece;

    pub const W_K: Piece = Piece::WhiteKing;
    pub const W_Q: Piece = Piece::WhiteQueen;
    pub const W_B: Piece = Piece::WhiteBishop;
    pub const W_N: Piece = Piece::WhiteKnight;
    pub const W_R: Piece = Piece::WhiteRook;
    pub const W_P: Piece = Piece::WhitePawn;

    pub const B_K: Piece = Piece::BlackKing;
    pub const B_Q: Piece = Piece::BlackQueen;
    pub const B_B: Piece = Piece::BlackBishop;
    pub const B_N: Piece = Piece::BlackKnight;
    pub const B_R: Piece = Piece::BlackRook;
    pub const B_P: Piece = Piece::BlackPawn;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PieceType::*;
        fmt::Display::fmt(&match *self {
            King => '♚',
            Queen => '♛',
            Bishop => '♝',
            Knight => '♞',
            Rook => '♜',
            Pawn => '♟',
        }, f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    WhiteKing,
    WhiteQueen,
    WhiteBishop,
    WhiteKnight,
    WhiteRook,
    WhitePawn,

    BlackKing,
    BlackQueen,
    BlackBishop,
    BlackKnight,
    BlackRook,
    BlackPawn,
}

impl Piece {
    pub const fn color(self) -> Color {
        use Piece::*;
        match self {
            WhiteKing |
            WhiteQueen |
            WhiteBishop |
            WhiteKnight |
            WhiteRook |
            WhitePawn => Color::White,

            BlackKing |
            BlackQueen |
            BlackBishop |
            BlackKnight |
            BlackRook |
            BlackPawn => Color::Black,
        }
    }

    pub const fn type_(self) -> PieceType {
        use Piece::*;
        match self {
            WhiteKing |
            BlackKing => PieceType::King,

            WhiteQueen |
            BlackQueen => PieceType::Queen,

            WhiteBishop |
            BlackBishop => PieceType::Bishop,

            WhiteKnight |
            BlackKnight => PieceType::Knight,

            WhiteRook |
            BlackRook => PieceType::Rook,

            WhitePawn |
            BlackPawn => PieceType::Pawn,
        }
    }

    /// Returns the unicode character corresponding to this piece
    pub const fn to_char(self) -> char {
        use Piece::*;
        match self {
            WhiteKing => '♔',
            WhiteQueen => '♕',
            WhiteBishop => '♗',
            WhiteKnight => '♘',
            WhiteRook => '♖',
            WhitePawn => '♙',

            BlackKing => '♚',
            BlackQueen => '♛',
            BlackBishop => '♝',
            BlackKnight => '♞',
            BlackRook => '♜',
            BlackPawn => '♟',
        }
    }
}

impl From<Piece> for char {
    fn from(piece: Piece) -> Self {
        piece.to_char()
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch: char = (*self).into();
        fmt::Display::fmt(&ch, f)
    }
}
