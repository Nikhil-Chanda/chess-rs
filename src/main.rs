mod game;
mod board;
mod piece;
mod movegen;
mod utils;

use std::io::{self, Write};
use game::Game;
use utils::parse_square;
use movegen::{ChessMove, is_legal_move};
// use piece::Color;

fn main() {
    let mut game = Game::new();
    loop {
        game.board.print();
        println!("{:?} to move", game.turn);
        print!("Enter move: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts:Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 2 {
            println!("Invalid input");
            continue;
        }

        let from = match parse_square(parts[0]) {
            Some(sq) => sq,
            None => {
                println!("Invalid source square");
                continue;
            },
        };
        let to = match parse_square(parts[1]) {
            Some(sq) => sq,
            None => {
                println!("Invalid destination square");
                continue;
            },
        };

        let piece = match game.board.get(from.0, from.1) {
            Some(q) => q,
            None => {
                println!("No piece in source square");
                continue;
            }
        };
        if piece.color != game.turn {
            println!("Not your piece");
            continue;
        }

        let mv = ChessMove {from, to};

        if !is_legal_move(&game.board, &mv) {
            println!("Illegal move");
            continue;
        }

        game.board.apply_move(mv);
        game.switch_turn();
    }
}