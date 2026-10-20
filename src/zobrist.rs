use rand::RngExt;

use std::sync::OnceLock;

pub struct Zobrist {
    pub pieces: [[u64; 64]; 12], // [piece-index][square]
    pub side: u64,
    pub castling: [u64; 4],
    pub enpassant_file: [u64; 8],
}

static ZOBRIST: OnceLock<Zobrist> = OnceLock::new();

impl Zobrist {
    pub fn new() -> Zobrist {
        let mut rng = rand::rng();
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
