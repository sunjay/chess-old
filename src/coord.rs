#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Represents the transformation (Rank+1, File+0)
    N,
    /// Represents the transformation (Rank+1, File+1)
    NE,
    /// Represents the transformation (Rank+0, File+1)
    E,
    /// Represents the transformation (Rank-1, File+1)
    SE,
    /// Represents the transformation (Rank-1, File+0)
    S,
    /// Represents the transformation (Rank-1, File-1)
    SW,
    /// Represents the transformation (Rank+0, File-1)
    W,
    /// Represents the transformation (Rank+1, File-1)
    NW,
}

/// A rank/row of the chess board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

/// A file/column of the chess board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

/// A coordinate representing the location of a tile on the chess board
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub file: File,
    pub rank: Rank,
}
