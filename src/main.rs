mod bitboard;
mod board;

use crate::{
    bitboard::{Color, PieceType, Square},
    board::Board,
};
fn main() {
    let mut board =
        Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq f6 0 3")
            .unwrap();

    println!("{:?}\n", board);
    // board.put(Color::White, PieceType::Knight, Square::new(0, 1)); // b1
    // board.put(Color::White, PieceType::Knight, Square::new(0, 6)); // g1
    // board.put(Color::Black, PieceType::Knight, Square::new(7, 6)); // g8
    // board.put(Color::Black, PieceType::King, Square::new(7, 4)); // e8

    board.print();
}
