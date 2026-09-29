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
    // let mut board =
    //     Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    //         .unwrap();

    // println!("{:?}\n", board);
    // board.print();

    // attack::knight_attacks(Square::new(3, 3)).print();
    // attack::knight_attacks(Square::new(0, 0)).print();
    // attack::knight_attacks(Square::new(7, 7)).print();
    //
    // attack::king_attacks(Square::new(0, 3)).print();
    // attack::king_attacks(Square::new(7, 4)).print();
    // attack::king_attacks(Square::new(3, 3)).print();
    //
    // attack::pawn_attacks(Color::White, Square::new(3, 3)).print();
    // attack::pawn_attacks(Color::Black, Square::new(3, 3)).print();
    // attack::pawn_attacks(Color::White, Square::new(1, 0)).print();
    // attack::pawn_attacks(Color::White, Square::new(1, 7)).print();

    // let mut occ = Bitboard::EMPTY;
    // occ.set(Square::new(5, 3));
    // occ.set(Square::new(5, 5));
    // attack::queen_attacks(Square::new(3, 3), occ).print();
    let board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

    let moves = Move::generate(&board);

    for m in &moves {
        println!("{}{}", m.from.to_algebraic(), m.to.to_algebraic());
    }
}
