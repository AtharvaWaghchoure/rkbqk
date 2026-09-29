use crate::{
    attack::{
        bishop_attacks, king_attacks, knight_attacks, pawn_attacks, queen_attacks, rook_attacks,
    },
    bitboard::{Bitboard, Color, PieceType, Square},
};

#[derive(Debug)]
pub struct Board {
    pub pieces: [[Bitboard; 6]; 2],
    pub side_to_move: Color,
    pub en_passant_target: Option<Square>,
    halfmove_clock: u8,
    fullmove_number: u16,
    /// if permitted to castle not can i castle next move.
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

impl Board {
    /// init board
    pub fn empty() -> Board {
        Board {
            pieces: [[Bitboard::EMPTY; 6]; 2],
            side_to_move: Color::White,
            en_passant_target: None,
            halfmove_clock: 0,
            fullmove_number: 1,
            white_kingside: false,
            white_queenside: false,
            black_kingside: false,
            black_queenside: false,
        }
    }

    /// all occupied squares on board for one color
    pub fn occupied_by(&self, color: Color) -> Bitboard {
        // self.pieces[color.index()][PieceType::Pawn.index()]
        //     | self.pieces[color.index()][PieceType::Knight.index()]
        //     | self.pieces[color.index()][PieceType::Bishop.index()]
        //     | self.pieces[color.index()][PieceType::Rook.index()]
        //     | self.pieces[color.index()][PieceType::Queen.index()]
        //     | self.pieces[color.index()][PieceType::King.index()]
        PieceType::ALL
            .iter()
            .fold(Bitboard::EMPTY, |bitboard, piecetype| {
                bitboard | self.pieces[color.index()][piecetype.index()]
            })
    }

    /// all occupied squares on board
    pub fn occupied(&self) -> Bitboard {
        self.occupied_by(Color::White) | self.occupied_by(Color::Black)
    }

    /// parsing a letter for color and piece type
    pub fn parse_piece(piece: char) -> Option<(Color, PieceType)> {
        let color = if piece.is_ascii_lowercase() {
            Color::Black
        } else {
            Color::White
        };
        let piece_type = match piece.to_ascii_lowercase() {
            'p' => Some(PieceType::Pawn),
            'n' => Some(PieceType::Knight),
            'b' => Some(PieceType::Bishop),
            'r' => Some(PieceType::Rook),
            'q' => Some(PieceType::Queen),
            'k' => Some(PieceType::King),
            _ => None,
        }?;
        Some((color, piece_type))
    }

    /// Board from fen string -> FEN Parser
    pub fn from_fen(fen: &str) -> Result<Board, String> {
        let mut board = Board::empty();
        let mut parts = fen.split_whitespace();
        let board_fields = parts.next().ok_or("missing board_fields")?;
        let side_to_move_field = parts.next().ok_or("missing side to move")?;
        let castling_field = parts.next().ok_or("missing castling field")?;
        let en_passant_field = parts.next().ok_or("missing en passant target field")?;
        let halfmove_field = parts.next().ok_or("missing halfmove field")?;
        let fullmove_field = parts.next().ok_or("missing fullmove field")?;

        // ranks
        for (rank, rank_str) in board_fields.split('/').rev().enumerate() {
            let mut file_cursor: u8 = 0;
            for piece in rank_str.chars() {
                if let Some(n) = piece.to_digit(10) {
                    file_cursor += n as u8;
                    continue;
                } else {
                    let (color, piece_type) = Self::parse_piece(piece).ok_or("bad piece char")?;
                    let sq = Square::new(rank as u8, file_cursor);
                    board.put(color, piece_type, sq);
                    file_cursor += 1;
                }
            }
        }

        // side to move
        board.side_to_move = match side_to_move_field {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("invalid side to move".into()),
        };

        // castling
        if castling_field
            .chars()
            .all(|c| matches!(c, 'K' | 'Q' | 'k' | 'q' | '-'))
        {
            board.white_kingside = castling_field.contains("K");
            board.white_queenside = castling_field.contains("Q");
            board.black_kingside = castling_field.contains("k");
            board.black_queenside = castling_field.contains("q");
        } else {
            return Err("bad castling char".into());
        }

        // en passant target
        board.en_passant_target = match en_passant_field {
            "-" => None,
            _ => Some(Square::from_str(en_passant_field)?),
        };

        // halfmove clock
        board.halfmove_clock = halfmove_field
            .parse()
            .map_err(|e| format!("halfmove error: {}", e))?;

        // fullmove clock
        board.fullmove_number = fullmove_field
            .parse()
            .map_err(|e| format!("fullmove error: {}", e))?;

        return Ok(board);
    }

    // putting a piece at a square
    pub fn put(&mut self, color: Color, piece_type: PieceType, sq: Square) {
        self.pieces[color.index()][piece_type.index()].set(sq);
    }

    // inverse of put i.e getting piece at a specific square
    pub fn piece_at(&self, sq: Square) -> Option<(Color, PieceType)> {
        for color in Color::ALL {
            for piece in PieceType::ALL {
                if self.pieces[color.index()][piece.index()].contains(sq) {
                    return Some((color, piece));
                }
            }
        }
        None
    }

    pub fn is_attacked(&self, sq: Square, by: Color) -> bool {
        let occupancy = self.occupied();
        let enemy = self.pieces[by.index()]; // 6 bitboard of attacking side.
        let hits = knight_attacks(sq) & enemy[PieceType::Knight.index()]
            | king_attacks(sq) & enemy[PieceType::King.index()]
            | bishop_attacks(sq, occupancy)
                & (enemy[PieceType::Bishop.index()] | enemy[PieceType::Queen.index()])
            | rook_attacks(sq, occupancy)
                & (enemy[PieceType::Rook.index()] | enemy[PieceType::Queen.index()])
            | pawn_attacks(by.flip(), sq) & enemy[PieceType::Pawn.index()];
        !hits.is_empty()
    }

    // pretty obvious
    pub fn print(&self) {
        for rank in (0..8).rev() {
            print!("{}  ", rank + 1);
            for file in 0..8 {
                let sq = Square::new(rank, file);
                let piece = self.piece_at(sq);
                match piece {
                    None => print!(". "),
                    Some((c, pc)) => match c {
                        Color::White => print!("{} ", pc.piece_letter()),
                        Color::Black => print!("{} ", pc.piece_letter().to_ascii_lowercase()),
                    },
                }
            }
            println!()
        }
        println!();
        println!("   a b c d e f g h");
    }
}
