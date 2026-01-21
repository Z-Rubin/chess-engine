use crate::board::board::Board;
use crate::board::piece::{Piece, Color};

const PAWN: i32 = 100;
const KNIGHT: i32 = 320;
const BISHOP: i32 = 330;
const ROOK: i32 = 500;
const QUEEN: i32 = 900;

pub fn evaluate(board: &Board) -> i32{
    let mut score = 0;

    for color in [Color::White, Color::Black] {
        let sign = if color == board.side_to_move { 1 } else { -1 };

        score += sign * material_score(board, color);
    }

    score
}

fn material_score(board: &Board, color: Color) -> i32 {
    let pieces = &board.pieces[color.index()];

    pieces[Piece::Pawn.index()].count_ones() as i32 * PAWN +
    pieces[Piece::Knight.index()].count_ones() as i32 * KNIGHT +
    pieces[Piece::Bishop.index()].count_ones() as i32 * BISHOP +
    pieces[Piece::Rook.index()].count_ones() as i32 * ROOK +
    pieces[Piece::Queen.index()].count_ones() as i32 * QUEEN
}