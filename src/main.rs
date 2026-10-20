mod attack;
mod bitboard;
mod board;
mod eval;
mod moves;
mod search;
mod zobrist;

use std::io::{self, Write};

use crate::{
    bitboard::{Bitboard, Color, Square},
    board::Board,
    moves::Move,
    search::{minimax, search_iterative, search_root_alphabeta, search_root_minimax},
};
fn main() {
    let mut board =
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    loop {
        board.print();
        let moves = board.legal_moves();
        if moves.is_empty() {
            if board.is_attacked(
                board.king_square(board.side_to_move),
                board.side_to_move.flip(),
            ) {
                println!(
                    "{:?} checkmates {:?}, {:?} wins",
                    board.side_to_move.flip(),
                    board.side_to_move,
                    board.side_to_move.flip()
                );
                break;
            } else {
                println!("Stalemate");
                break;
            }
        }

        if board.side_to_move == Color::White {
            print!("your move: ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            let n = io::stdin().read_line(&mut line).unwrap();
            if n == 0 {
                println!("bye");
                break;
            }
            let input = line.trim();
            match parse_move(input, &mut board) {
                Some(mv) => board.make_move(mv),
                None => {
                    println!("illegal, retry");
                    continue;
                }
            };
        } else {
            let mv = search_iterative(&mut board, 4).unwrap().0;
            println!(
                "engine plays {}{}",
                mv.from.to_algebraic(),
                mv.to.to_algebraic()
            );
            board.make_move(mv);
        }
    }

    // let mut b = Board::from_fen("6k1/5ppp/8/8/8/8/8/R6K w - - 0 1").unwrap();
    // let mut b1 = Board::from_fen("4k3/8/2p5/3p4/8/8/8/3QK3 w - - 0 1").unwrap();
    // let Some((mv, score)) = search_iterative(&mut b, 5) else {
    //     println!("table corrupted");
    //     return;
    // };
    // let Some((mv1, score1)) = search_iterative(&mut b1, 5) else {
    //     println!("table corrupted");
    //     return;
    // };
    // println!(
    //     "Best move: {}{}  (score {})",
    //     mv.from.to_algebraic(),
    //     mv.to.to_algebraic(),
    //     score
    // );
    // println!(
    //     "Best move: {}{}  (score {})",
    //     mv1.from.to_algebraic(),
    //     mv1.to.to_algebraic(),
    //     score1
    // );
}

fn parse_move(input: &str, board: &mut Board) -> Option<Move> {
    if input.len() < 4 {
        return None;
    }
    let from = Square::from_str(&input[0..2]).ok()?;
    let to = Square::from_str(&input[2..4]).ok()?;

    board
        .legal_moves()
        .into_iter()
        .find(|mv| mv.from == from && mv.to == to)
}
