use crate::{
    attack::{
        bishop_attacks, king_attacks, knight_attacks, pawn_attacks, queen_attacks, rook_attacks,
    },
    bitboard::{Color, PieceType, Square},
    board::Board,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MoveKind {
    Quiet,
    DoublePush,
    EnPassant,
    Castle,
    Promotion(PieceType),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub kind: MoveKind,
}

impl Move {
    pub fn generate(board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        let side_to_move = board.side_to_move;
        let own_pieces = board.occupied_by(side_to_move);
        let them_pieces = board.occupied_by(side_to_move.flip());

        //  non-sliders
        let mut knights = board.pieces[side_to_move.index()][PieceType::Knight.index()];
        while !knights.is_empty() {
            let from = knights.pop_lsb();
            let mut targets = knight_attacks(from) & !own_pieces;
            while !targets.is_empty() {
                let to = targets.pop_lsb();
                moves.push(Move {
                    from,
                    to,
                    kind: MoveKind::Quiet,
                });
            }
        }

        let mut kings = board.pieces[side_to_move.index()][PieceType::King.index()];
        while !kings.is_empty() {
            let from = kings.pop_lsb();
            let mut targets = king_attacks(from) & !own_pieces;
            while !targets.is_empty() {
                let to = targets.pop_lsb();
                moves.push(Move {
                    from,
                    to,
                    kind: MoveKind::Quiet,
                });
            }
        }
        if side_to_move == Color::Black {
            let e8 = Square::new(7, 4);

            let f8 = Square::new(7, 5);
            let g8 = Square::new(7, 6);

            let d8 = Square::new(7, 3);
            let c8 = Square::new(7, 2);
            let b8 = Square::new(7, 1);

            // kingside
            if board.black_kingside
                && !board.occupied().contains(f8)
                && !board.occupied().contains(g8)
                && !board.is_attacked(e8, side_to_move.flip())
                && !board.is_attacked(f8, side_to_move.flip())
                && !board.is_attacked(g8, side_to_move.flip())
            {
                moves.push(Move {
                    from: e8,
                    to: g8,
                    kind: MoveKind::Castle,
                });
            }
            // Queen side
            if board.black_queenside
                && !board.occupied().contains(d8)
                && !board.occupied().contains(c8)
                && !board.occupied().contains(b8)
                && !board.is_attacked(e8, side_to_move.flip())
                && !board.is_attacked(d8, side_to_move.flip())
                && !board.is_attacked(c8, side_to_move.flip())
            {
                moves.push(Move {
                    from: e8,
                    to: c8,
                    kind: MoveKind::Castle,
                });
            }
        }

        if side_to_move == Color::White {
            let e1 = Square::new(0, 4);

            let f1 = Square::new(0, 5);
            let g1 = Square::new(0, 6);

            let d1 = Square::new(0, 3);
            let c1 = Square::new(0, 2);
            let b1 = Square::new(0, 1);

            // kingside
            if board.white_kingside
                && !board.occupied().contains(f1)
                && !board.occupied().contains(g1)
                && !board.is_attacked(e1, side_to_move.flip())
                && !board.is_attacked(f1, side_to_move.flip())
                && !board.is_attacked(g1, side_to_move.flip())
            {
                moves.push(Move {
                    from: e1,
                    to: g1,
                    kind: MoveKind::Castle,
                });
            }
            // Queen side
            if board.white_queenside
                && !board.occupied().contains(d1)
                && !board.occupied().contains(c1)
                && !board.occupied().contains(b1)
                && !board.is_attacked(e1, side_to_move.flip())
                && !board.is_attacked(d1, side_to_move.flip())
                && !board.is_attacked(c1, side_to_move.flip())
            {
                moves.push(Move {
                    from: e1,
                    to: c1,
                    kind: MoveKind::Castle,
                });
            }
        }

        // sliders

        let mut rook = board.pieces[side_to_move.index()][PieceType::Rook.index()];
        while !rook.is_empty() {
            let from = rook.pop_lsb();
            let mut targets = rook_attacks(from, board.occupied()) & !own_pieces;
            while !targets.is_empty() {
                let to = targets.pop_lsb();
                moves.push(Move {
                    from,
                    to,
                    kind: MoveKind::Quiet,
                });
            }
        }

        let mut bishop = board.pieces[side_to_move.index()][PieceType::Bishop.index()];
        while !bishop.is_empty() {
            let from = bishop.pop_lsb();
            let mut targets = bishop_attacks(from, board.occupied()) & !own_pieces;
            while !targets.is_empty() {
                let to = targets.pop_lsb();
                moves.push(Move {
                    from,
                    to,
                    kind: MoveKind::Quiet,
                });
            }
        }

        let mut queen = board.pieces[side_to_move.index()][PieceType::Queen.index()];
        while !queen.is_empty() {
            let from = queen.pop_lsb();
            let mut targets = queen_attacks(from, board.occupied()) & !own_pieces;
            while !targets.is_empty() {
                let to = targets.pop_lsb();
                moves.push(Move {
                    from,
                    to,
                    kind: MoveKind::Quiet,
                });
            }
        }

        // Pawn

        let mut pawn = board.pieces[side_to_move.index()][PieceType::Pawn.index()];
        while !pawn.is_empty() {
            let from = pawn.pop_lsb();
            let ahead;
            if side_to_move == Color::White {
                let ahead_rank = from.rank() + 1;
                ahead = Square::new(ahead_rank, from.file())
            } else {
                let ahead_rank = from.rank() - 1;
                ahead = Square::new(ahead_rank, from.file())
            }

            // single push
            if !board.occupied().contains(ahead) {
                Self::push_pawn_move(&mut moves, from, ahead, side_to_move);
                // moves.push(Move {
                //     from,
                //     to: ahead,
                //     kind: MoveKind::Quiet,
                // });

                // double push
                if side_to_move == Color::White && from.rank() == 1 {
                    let two_ahead_rank = from.rank() + 2;
                    let two_ahead = Square::new(two_ahead_rank, from.file());
                    if !board.occupied().contains(two_ahead) {
                        moves.push(Move {
                            from,
                            to: two_ahead,
                            kind: MoveKind::DoublePush,
                        });
                    }
                } else if side_to_move == Color::Black && from.rank() == 6 {
                    let two_ahead_rank = from.rank() - 2;
                    let two_ahead = Square::new(two_ahead_rank, from.file());
                    if !board.occupied().contains(two_ahead) {
                        moves.push(Move {
                            from,
                            to: two_ahead,
                            kind: MoveKind::DoublePush,
                        });
                    }
                }
            }

            // pawn capture
            let mut captures = pawn_attacks(side_to_move, from) & them_pieces;
            while !captures.is_empty() {
                let to = captures.pop_lsb();
                Self::push_pawn_move(&mut moves, from, to, side_to_move);
                // moves.push(Move {
                //     from,
                //     to,
                //     kind: MoveKind::Quiet,
                // });
            }

            // en passant
            if let Some(ep_sq) = board.en_passant_target {
                if pawn_attacks(side_to_move, from).contains(ep_sq) {
                    moves.push(Move {
                        from,
                        to: ep_sq,
                        kind: MoveKind::EnPassant,
                    });
                }
            }
        }

        return moves;
    }
    fn push_pawn_move(moves: &mut Vec<Move>, from: Square, to: Square, color: Color) {
        if to.rank() == 7 && color == Color::White {
            for piece in [
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
            ] {
                moves.push(Move {
                    from,
                    to,
                    kind: MoveKind::Promotion(piece),
                });
            }
        } else if to.rank() == 0 && color == Color::Black {
            for piece in [
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
            ] {
                moves.push(Move {
                    from,
                    to,
                    kind: MoveKind::Promotion(piece),
                });
            }
        } else {
            moves.push(Move {
                from,
                to,
                kind: MoveKind::Quiet,
            });
        }
    }
}
