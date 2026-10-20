use rand::{RngExt, SeedableRng, rngs::StdRng};

use std::sync::OnceLock;

use crate::moves::Move;

#[derive(Clone, Copy)]
pub enum Bound {
    Exact,
    Lower,
    Upper,
}

#[derive(Clone, Copy)]
pub struct TtEntry {
    pub key: u64, // full hash - to verify the slot is really this position
    pub depth: u32,
    pub score: i32,
    pub bound: Bound,
    pub best: Option<Move>,
}

/// Transposition Table
pub struct Tt {
    table: Vec<Option<TtEntry>>,
}

impl Tt {
    pub fn new(size: usize) -> Tt {
        assert!(
            size.is_power_of_two(),
            "Transposition Table size must be power of two"
        );
        Tt {
            table: vec![None; size],
        }
    }
    pub fn probe(&self, hash: u64, depth: u32, alpha: i32, beta: i32) -> Option<i32> {
        let idx = (hash as usize) & (self.table.len() - 1);
        let Some(e) = &self.table[idx] else {
            return None;
        };
        if e.key != hash || e.depth < depth {
            return None;
        };
        match e.bound {
            Bound::Exact => return Some(e.score),
            Bound::Lower if e.score >= beta => Some(e.score),
            Bound::Upper if e.score <= alpha => Some(e.score),
            _ => None,
        }
    }

    pub fn store(&mut self, hash: u64, depth: u32, score: i32, bound: Bound, best: Option<Move>) {
        let idx = (hash as usize) & (self.table.len() - 1);
        self.table[idx] = Some(TtEntry {
            key: hash,
            depth,
            score,
            bound,
            best,
        })
    }
}
pub struct Zobrist {
    pub pieces: [[u64; 64]; 12], // [piece-index][square]
    pub side: u64,
    pub castling: [u64; 4],
    pub enpassant_file: [u64; 8],
}

static ZOBRIST: OnceLock<Zobrist> = OnceLock::new();

impl Zobrist {
    pub fn new() -> Zobrist {
        let mut rng = StdRng::seed_from_u64(0xC0FFEE);
        let mut pieces = [[0u64; 64]; 12];
        for row in &mut pieces {
            for x in row {
                *x = rng.random();
            }
        }

        let mut castling = [0u64; 4];
        for x in &mut castling {
            *x = rng.random();
        }

        let mut enpassant_file = [0u64; 8];
        for x in &mut enpassant_file {
            *x = rng.random();
        }

        Zobrist {
            pieces,
            side: rng.random(),
            castling,
            enpassant_file,
        }
    }
    pub fn zobrist() -> &'static Zobrist {
        ZOBRIST.get_or_init(Zobrist::new)
    }
}
