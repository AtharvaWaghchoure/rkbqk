use std::ops::BitOr;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const ALL: [Color; 2] = [Color::White, Color::Black];
    pub fn flip(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
    pub fn index(self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 1,
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

impl PieceType {
    pub const ALL: [PieceType; 6] = [
        PieceType::Pawn,
        PieceType::Knight,
        PieceType::Bishop,
        PieceType::Rook,
        PieceType::Queen,
        PieceType::King,
    ];
    pub fn index(self) -> usize {
        match self {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5,
        }
    }

    pub fn piece_letter(self) -> char {
        match self {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

    pub fn from_str(to_sq: &str) -> Result<Square, String> {
        if to_sq.len() == 2 {
            let bytes = to_sq.as_bytes();
            let file;
            let rank;
            if matches!(bytes[0], b'a'..=b'h') {
                file = bytes[0] - b'a';
            } else {
                return Err("file string out of range".into());
            }
            if matches!(bytes[1], b'1'..=b'8') {
                rank = bytes[1] - b'1';
            } else {
                return Err("file string out of range".into());
            }
            Ok(Self::new(rank, file))
        } else {
            return Err("invalid square string".into());
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bitboard(pub u64);

impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

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
        println!("  a b c d e f g h");
    }
}
