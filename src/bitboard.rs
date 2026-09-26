#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn flip(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Square(pub u8);

impl Square {
    pub fn new(rank: u8, file: u8) -> Square {
        Square((rank * 8) + file)
    }
    pub fn rank(self) -> u8 {
        self.0 / 8
    }

    pub fn file(self) -> u8 {
        self.0 % 8
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn contains(self, sq: Square) -> bool {
        (self.0 >> sq.0) & 1 == 1
    }

    pub fn set(&mut self, sq: Square) {
        self.0 |= 1 << sq.0
    }

    pub fn clear(&mut self, sq: Square) {
        self.0 &= !(1 << sq.0)
    }

    pub fn pop_lsb(&mut self) -> Square {
        let index = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;
        Square(index)
    }

    pub fn print(self) {
        for rank in (0..8).rev() {
            print!("{} ", rank + 1);
            for file in 0..8 {
                let sq = Square::new(rank, file);
                print!("{} ", if self.contains(sq) { 'X' } else { '.' })
            }
            println!();
        }
        println!("   a b c d e f g h");
    }
}
