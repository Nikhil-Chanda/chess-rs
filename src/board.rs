use crate::piece::{Piece, PieceType, Color};
use crate::movegen::ChessMove;

pub type Square = Option<Piece>;

pub struct Board {
    pub squares:[[Square; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        Self {
            squares: [[None; 8]; 8],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Square {
        self.squares[row][col]
    }

    pub fn setup_standard(&mut self) {
        for col in 0..8 {
            self.squares[1][col] = Some(Piece {piece_type: PieceType::Pawn, color: Color::White});
            self.squares[6][col] = Some(Piece {piece_type: PieceType::Pawn, color: Color::Black});
        }

        let back_rank = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for col in 0..8 {
            self.squares[0][col] = Some(Piece {piece_type: back_rank[col], color: Color::White});
            self.squares[7][col] = Some(Piece {piece_type: back_rank[col], color: Color::Black});
        }
    }

    pub fn apply_move(&mut self, mv: ChessMove) {
        let piece = self.squares[mv.from.0][mv.from.1];
        self.squares[mv.from.0][mv.from.1] = None;
        self.squares[mv.to.0][mv.to.1] = piece;
    }

    pub fn print(&self) {
        for row in (0..8).rev() {
            print!("{} ", row+1);
            for col in 0..8 {
                match self.squares[row][col] {
                    Some(piece) => {
                        let c = match piece.piece_type {
                            crate::piece::PieceType::King => 'K',
                            crate::piece::PieceType::Queen => 'Q',
                            crate::piece::PieceType::Rook => 'R',
                            crate::piece::PieceType::Bishop => 'B',
                            crate::piece::PieceType::Knight => 'N',
                            crate::piece::PieceType::Pawn => 'P',
                        };
                        print!("{} ", c);
                    }
                    None => print!(". "),
                }
            }
            println!();
        }
        println!("  a b c d e f g h");
    }
}