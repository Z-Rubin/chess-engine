use crate::board::board::Board;
use crate::board::r#move::Move;
use crate::movegen::movegen::generate_legal_moves;
use crate::eval::eval::evaluate;

const INF: i32 = 1_000_000;

pub fn search(board: &mut Board, depth: u32) -> (i32, Option<Move>) {
    let mut best_score = -INF;
    let mut best_move = None;

    for mv in generate_legal_moves(board) {
        let undo = board.make_move(mv);
        let score = -negamax(board, depth - 1, -INF, INF);
        board.unmake_move(mv, undo);

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }
    }

    (best_score, best_move)
}

fn negamax(board: &mut Board, depth: u32, mut alpha: i32, beta: i32) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    let moves = generate_legal_moves(board);

    if moves.is_empty() {
        return if board.in_check(board.side_to_move) {
            -INF + depth as i32 // checkmate
        } else {
            0 // stalemate
        };
    }

    let mut best = -INF;

    for mv in moves {
        let undo = board.make_move(mv);
        let score = -negamax(board, depth - 1, -beta, -alpha);
        board.unmake_move(mv, undo);

        best = best.max(score);
        alpha = alpha.max(score);

        if alpha >= beta {
            break; // alpha-beta cutoff
        }
    }

    best
}
