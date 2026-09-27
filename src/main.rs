mod attack;
mod bitboard;
mod board;

use crate::{
    bitboard::{Bitboard, Color, Square},
    board::Board,
};
fn main() {
    // let mut board =
    //     Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq f6 0 3")
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

    let mut occ = Bitboard::EMPTY;
    occ.set(Square::new(5, 3));
    occ.set(Square::new(5, 5));
    attack::queen_attacks(Square::new(3, 3), occ).print();
}
