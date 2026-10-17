mod attack;
mod bitboard;
mod board;
mod eval;
mod moves;

use crate::{
    bitboard::{Bitboard, Color, Square},
    board::Board,
    eval::evaluate,
    moves::Move,
};
fn main() {
    let mut board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    println!("{}", evaluate(&board));
}
