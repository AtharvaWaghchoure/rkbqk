use crate::bitboard::{Bitboard, Color, Square};

// Non sliders
fn offset_attacks(sq: Square, deltas: &[(i32, i32)]) -> Bitboard {
    let r0 = sq.rank();
    let f0 = sq.file();
    let mut attack = Bitboard::EMPTY;
    for &(dr, df) in deltas {
        let r = r0 as i32 + dr;
        let f = f0 as i32 + df;
        if matches!(r, 0..=7) && matches!(f, 0..=7) {
            attack.set(Square::new(r as u8, f as u8));
        }
    }
    attack
}

pub fn knight_attacks(sq: Square) -> Bitboard {
    let deltas: [(i32, i32); 8] = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (-1, 2),
        (1, -2),
        (-1, -2),
    ];
    offset_attacks(sq, &deltas)
}

pub fn king_attacks(sq: Square) -> Bitboard {
    let deltas: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    offset_attacks(sq, &deltas)
}

pub fn pawn_attacks(color: Color, sq: Square) -> Bitboard {
    let white_detlas: [(i32, i32); 2] = [(1, 1), (1, -1)];
    let black_detlas: [(i32, i32); 2] = [(-1, 1), (-1, -1)];
    match color {
        Color::White => offset_attacks(sq, &white_detlas),
        Color::Black => offset_attacks(sq, &black_detlas),
    }
}

// Sliders

fn ray_attacks(sq: Square, occupancy: Bitboard, directions: &[(i32, i32)]) -> Bitboard {
    let mut attack = Bitboard::EMPTY;
    let r0 = sq.rank();
    let f0 = sq.file();

    for (dr, df) in directions {
        let mut r = r0 as i32;
        let mut f = f0 as i32;
        loop {
            r += dr;
            f += df;
            if !matches!(r, 0..=7) || !matches!(f, 0..=7) {
                break;
            }
            let s = Square::new(r as u8, f as u8);
            attack.set(s);
            if occupancy.contains(s) {
                break;
            }
        }
    }
    attack
}

pub fn rook_attacks(sq: Square, occupancy: Bitboard) -> Bitboard {
    let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    ray_attacks(sq, occupancy, &directions)
}

pub fn bishop_attacks(sq: Square, occupancy: Bitboard) -> Bitboard {
    let directions: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    ray_attacks(sq, occupancy, &directions)
}

pub fn queen_attacks(sq: Square, occupancy: Bitboard) -> Bitboard {
    rook_attacks(sq, occupancy) | bishop_attacks(sq, occupancy)
}
