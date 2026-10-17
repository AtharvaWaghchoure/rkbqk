mod attack;
mod bitboard;
mod board;
mod moves;

use crate::{
    bitboard::{Bitboard, Color, Square},
    board::Board,
    moves::Move,
};
fn main() {
    let mut board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    for d in 1..=4 {
        println!("perft({}) = {}", d, board.perft(d));
    }
}
