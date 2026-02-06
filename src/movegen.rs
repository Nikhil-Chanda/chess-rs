use crate::board::Board;
use crate::piece::{PieceType, Color};

pub struct ChessMove {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

pub fn in_bounds(r: isize, c: isize) -> bool {
    r >= 0 && r < 8 && c >= 0 && c < 8
}

pub fn pawn_moves(board: &Board, r: usize, c: usize) -> Vec<ChessMove> {
    let mut moves = Vec::new();
    let piece = board.get(r, c).unwrap();

    let dir: isize = if piece.color == Color::White { 1 } else { -1 };

    let nr = r as isize + dir;
    if in_bounds(nr, c as isize) && board.get(nr as usize, c).is_none() {
        moves.push(ChessMove {
            from: (r, c),
            to: (nr as usize, c),
        });
    }

    for dc in [-1, 1] {
        let nc = c as isize + dc;
        if in_bounds(nr, nc) {
            if let Some(target) = board.get(nr as usize, nc as usize) {
                if target.color != piece.color {
                    moves.push(ChessMove {
                        from: (r, c),
                        to: (nr as usize, nc as usize),
                    });
                }
            }
        }
    }

    moves
}

fn sliding_moves(board: &Board, r: usize, c: usize, directions: &[(isize, isize)]) -> Vec<ChessMove> {
    let mut moves = Vec::new();
    let piece = board.get(r, c).unwrap();

    for (dr, dc) in directions {
        let mut nr = r as isize + dr;
        let mut nc = c as isize + dc;

        while in_bounds(nr, nc) {
            match board.get(nr as usize, nc as usize) {
                None => {
                    moves.push(ChessMove {
                        from: (r, c),
                        to: (nr as usize, nc as usize),
                    });
                }
                Some(target) => {
                    if target.color != piece.color {
                        moves.push(ChessMove {
                            from: (r, c),
                            to: (nr as usize, nc as usize),
                        });
                    }
                    break;
                }
            }
        }
        nr += dr;
        nc += dc;
    }
    moves
}

pub fn rook_moves(board: &Board, r: usize, c: usize) -> Vec<ChessMove> {
    sliding_moves(board, r, c, &[(1, 0), (-1, 0), (0, 1), (0, -1)])
}

pub fn bishop_moves(board: &Board, r: usize, c: usize) -> Vec<ChessMove> {
    sliding_moves(board, r, c, &[(1, 1), (-1, 1), (1, -1), (-1, -1)])
}

pub fn queen_moves(board: &Board, r: usize, c: usize) -> Vec<ChessMove> {
    sliding_moves(board, r, c, &[(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)])
}

pub fn knight_moves(board: &Board, r: usize, c: usize) -> Vec<ChessMove> {
    let mut moves = Vec::new();
    let piece = board.get(r, c).unwrap();

    let deltas = [(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2)];

    for (dr, dc) in deltas {
        let mut nr = r as isize + dr;
        let mut nc = c as isize + dc;

        if in_bounds(nr, nc) {
            match board.get(nr as usize, nc as usize) {
                None => moves.push(ChessMove {
                    from: (r, c),
                    to: (nr as usize, nc as usize),
                }),
                Some(target) if target.color != piece.color => moves.push(ChessMove {
                    from: (r, c),
                    to: (nr as usize, nc as usize),
                }),
                _ => {}
            }
        }
    }
    moves
}

pub fn king_moves(board: &Board, r: usize, c: usize) -> Vec<ChessMove> {
    let mut moves = Vec::new();
    let piece = board.get(r, c).unwrap();

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 { continue; }
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if in_bounds(nr, nc) {
                match board.get(nr as usize, nc as usize) {
                    None => moves.push(ChessMove {
                        from: (r, c),
                        to: (nr as usize, nc as usize),
                    }),
                    Some(target) if target.color != piece.color => moves.push(ChessMove {
                        from: (r, c),
                        to: (nr as usize, nc as usize),
                    }),
                    _ => {}
                }
            }
        }
    }
    moves
}

pub fn generate_moves(board: &Board, r: usize, c: usize) -> Vec<ChessMove> {
    let piece = match board.get(r, c) {
        Some(p) => p,
        None => return vec![],
    };

    match piece.piece_type {
        PieceType::Pawn => pawn_moves(board, r, c),
        PieceType::Rook => rook_moves(board, r, c),
        PieceType::Bishop => bishop_moves(board, r, c),
        PieceType::Queen => queen_moves(board, r, c),
        PieceType::Knight => knight_moves(board, r, c),
        PieceType::King => king_moves(board, r, c),
    }
}

pub fn is_legal_move(board: &Board, mv: &ChessMove) -> bool{
    generate_moves(board, mv.from.0, mv.from.1).iter().any(|m| m.to == mv.to)
}