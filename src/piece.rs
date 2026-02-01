#[derive(Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}