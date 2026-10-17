use crate::{bitboard::Color, board::Board, eval::evaluate, moves::Move};

pub fn minimax(board: &mut Board, depth: u32, nodes: &mut u64) -> i32 {
    *nodes += 1;
    if depth == 0 {
        return evaluate(board);
    }

    let moves = board.legal_moves();

    if moves.is_empty() {
        return evaluate(board);
    }

    let maximizing = board.side_to_move == Color::White;
    let mut best = if maximizing { i32::MIN } else { i32::MAX };
    for mv in &moves {
        let undo = board.make_move(*mv);
        let score = minimax(board, depth - 1, nodes);
        best = if maximizing {
            best.max(score)
        } else {
            best.min(score)
        };
        board.unmake_move(*mv, undo);
    }
    best
}

pub fn search_root(board: &mut Board, depth: u32) -> (Option<Move>, i32) {
    let mut best_move = None;
    let maximizing = board.side_to_move == Color::White;
    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
    let mut nodes = 0u64;
    for mv in board.legal_moves() {
        let undo = board.make_move(mv);
        let mut score = minimax(board, depth - 1, &mut nodes);
        board.unmake_move(mv, undo);
        let better = if maximizing {
            score > best_score
        } else {
            score < best_score
        };

        if better {
            best_score = score;
            best_move = Some(mv);
        }
    }
    return (best_move, best_score);
}
