use crate::board::Board;
use crate::piece::{PieceType, Color};

pub struct ChessMove {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

pub fn in_bounds(r: isize, c: isize) -> bool {
    r >= 0 && r < 8 && c >= 0 && c < 8
}

pub fn pawn_moves(board: &Board, r: isize, c: isize) -> Vec<ChessMove> {
    let mut moves = Vec::new();
    let piece = board.get(r, c).unwrap();

    let dir: isize = if piece::color == Color::White { 1 } else { -1 };

    let nr = r as isize + dir;
    if in_bounds(nr, c as isize) && board.get(nr as usize, c) {
        
    }
}

pub fn is_legal_move(board: &Board, mv: &ChessMove) -> bool{
    let piece = match board.get(mv.from.0, mv.from.1) {
        Some(p) => p,
        None => return false,
    };

    match piece.piece_type {
        PieceType::Pawn => pawn_moves(board, r, c),
        _ => true,
    }
}