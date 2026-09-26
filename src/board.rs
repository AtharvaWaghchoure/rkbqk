use crate::bitboard::{Bitboard, Color, PieceType, Square};

#[derive(Debug)]
pub struct Board {
    pieces: [[Bitboard; 6]; 2],
    side_to_move: Color,
    en_passant_target: Option<Square>,
    halfmove_clock: u8,
    fullmove_number: u16,
    /// if permitted to castle not can i castle next move.
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}

impl Board {
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

        board.side_to_move = match side_to_move_field {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("invalid side to move".into()),
        };

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

        board.en_passant_target = match en_passant_field {
            "-" => None,
            _ => Some(Square::from_str(en_passant_field)?),
        };

        board.halfmove_clock = halfmove_field
            .parse()
            .map_err(|e| format!("halfmove error: {}", e))?;

        board.fullmove_number = fullmove_field
            .parse()
            .map_err(|e| format!("fullmove error: {}", e))?;

        return Ok(board);
    }

    pub fn put(&mut self, color: Color, piece_type: PieceType, sq: Square) {
        self.pieces[color.index()][piece_type.index()].set(sq);
    }

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
