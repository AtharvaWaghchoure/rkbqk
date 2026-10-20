use crate::{
    bitboard::Color,
    board::Board,
    eval::{VALUES, eval_stm},
    moves::Move,
};

const INF: i32 = 32_000;
const MATE: i32 = 30_000;

fn terminal_score(board: &Board, ply: u32) -> i32 {
    let in_check = board.is_attacked(
        board.king_square(board.side_to_move),
        board.side_to_move.flip(),
    );
    if in_check {
        -(MATE - ply as i32)
    } else {
        return 0;
    }
}
pub fn minimax(board: &mut Board, depth: u32, nodes: &mut u64, ply: u32) -> i32 {
    *nodes += 1;
    if depth == 0 {
        return eval_stm(board);
    }

    let moves = board.legal_moves();

    if moves.is_empty() {
        return terminal_score(board, ply);
    }

    let maximizing = board.side_to_move == Color::White;
    let mut best = if maximizing { i32::MIN } else { i32::MAX };
    for mv in &moves {
        let undo = board.make_move(*mv);
        let score = minimax(board, depth - 1, nodes, ply + 1);
        best = if maximizing {
            best.max(score)
        } else {
            best.min(score)
        };
        board.unmake_move(*mv, undo);
    }
    best
}

pub fn alphabeta(
    board: &mut Board,
    depth: u32,
    mut alpha: i32,
    mut beta: i32,
    nodes: &mut u64,
    ply: u32,
) -> i32 {
    *nodes += 1;
    if depth == 0 {
        return quiescence(board, alpha, beta, nodes);
    }

    let mut moves = board.legal_moves();

    if moves.is_empty() {
        return terminal_score(board, ply);
    }

    moves.sort_by_key(|mv| {
        -(match board.piece_at(mv.to) {
            Some((_, victim)) => {
                (VALUES[victim.index()] * 10) - VALUES[board.piece_at(mv.from).unwrap().1.index()]
            }
            _ => 0,
        })
    });

    let mut best = -INF;
    for mv in &moves {
        let undo = board.make_move(*mv);
        let score = -alphabeta(board, depth - 1, -beta, -alpha, nodes, ply + 1);
        board.unmake_move(*mv, undo);
        best = best.max(score);
        alpha = alpha.max(best);
        if alpha >= beta {
            break;
        }
    }
    best
}

pub fn search_iterative(board: &mut Board, max_depth: u32) -> Option<(Move, i32)> {
    let mut best = None;
    for d in 1..=max_depth {
        let (mv, score, nodes) = search_root_alphabeta(board, d);
        best = Some((mv.unwrap(), score));
        println!(
            "depth: {d} {}{} score {score} nodes {nodes}",
            mv.unwrap().from.to_algebraic(),
            mv.unwrap().to.to_algebraic()
        );
    }
    best
}

pub fn quiescence(board: &mut Board, mut alpha: i32, mut beta: i32, nodes: &mut u64) -> i32 {
    let mut stand_pat = eval_stm(board);

    if stand_pat >= beta {
        return stand_pat;
    }
    alpha = alpha.max(stand_pat);

    let mut captures = board.legal_moves();
    captures.retain(|mv| board.piece_at(mv.to).is_some());

    for capture in captures {
        let undo = board.make_move(capture);
        let score = -quiescence(board, -beta, -alpha, nodes);
        stand_pat = stand_pat.max(score);
        alpha = alpha.max(stand_pat);
        board.unmake_move(capture, undo);
        if alpha >= beta {
            break;
        }
    }
    stand_pat
}
pub fn search_root_minimax(board: &mut Board, depth: u32) -> (Option<Move>, i32) {
    let mut best_move = None;
    let maximizing = board.side_to_move == Color::White;
    let mut best_score = if maximizing { i32::MIN } else { i32::MAX };
    let mut nodes = 0u64;
    for mv in board.legal_moves() {
        let undo = board.make_move(mv);
        let score = minimax(board, depth - 1, &mut nodes, 1);
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
    (best_move, best_score)
}

pub fn search_root_alphabeta(board: &mut Board, depth: u32) -> (Option<Move>, i32, u64) {
    let mut best_move: Option<Move> = None;
    let mut best_score = -INF;
    let mut nodes = 0u64;
    for mv in board.legal_moves() {
        let undo = board.make_move(mv);
        let score = -alphabeta(board, depth - 1, -INF, INF, &mut nodes, 1);
        board.unmake_move(mv, undo);
        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }
    }
    (best_move, best_score, nodes)
}
