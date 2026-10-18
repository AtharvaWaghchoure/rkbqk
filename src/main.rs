mod attack;
mod bitboard;
mod board;
mod eval;
mod moves;
mod search;

use crate::{
    bitboard::{Bitboard, Color, Square},
    board::Board,
    eval::evaluate,
    moves::Move,
    search::{minimax, search_root_alphabeta, search_root_minimax},
};
fn main() {
    let mut board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let mut b = Board::from_fen("q3k3/8/8/8/8/8/8/R3K3 w - - 0 1").unwrap();
    let mut b1 = Board::from_fen("r3k3/8/8/8/8/8/4Q3/4K3 w - - 0 1").unwrap();

    let (mv, score) = search_root_alphabeta(&mut b1, 4);
    println!(
        "Best move: {}{}  (score {})",
        mv.unwrap().from.to_algebraic(),
        mv.unwrap().to.to_algebraic(),
        score
    );
}
