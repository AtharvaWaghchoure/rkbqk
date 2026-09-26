mod bitboard;

use bitboard::{Bitboard, Square};
fn main() {
    let mut bb = Bitboard::EMPTY;

    bb.set(Square::new(0, 1)); // b1
    bb.set(Square::new(0, 6)); // g1
    bb.print();
}
