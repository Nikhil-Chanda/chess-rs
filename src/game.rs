use crate::board::Board;
use crate::piece::Color;

pub struct Game {
    pub board: Board,
    pub turn: Color,
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::new();
        board.setup_standard();

        Self {
            board,
            turn: Color::White,
        }
    }

    pub fn switch_turn(&mut self) {
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}