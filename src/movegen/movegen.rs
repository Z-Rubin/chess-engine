
use crate::board::board::Board;
use crate::board::r#move::{Move, MoveList};
use crate::board::piece::{Color, Piece};
use crate::board::bitboard::*;
use crate::movegen::attacks::{
    rook_attacks,
    bishop_attacks,
    queen_attacks,
    knight_attacks,
    king_attacks,
};


#[inline]
fn pop_lsb(bb: &mut u64) -> u8 {
    let sq = bb.trailing_zeros() as u8;
    *bb &= *bb - 1;
    sq
}

pub fn gen_knights(board: &Board, moves: &mut MoveList) {
    let color = board.side_to_move;
    let knights = board.pieces[color.index()][Piece::Knight.index()];
    let own = board.occupied_by(color);

    let mut bb = knights;
    while bb != 0 {
        let from = pop_lsb(&mut bb);
        let mut targets = knight_attacks(from) & !own;

        while targets != 0 {
            let to = pop_lsb(&mut targets);
            moves.push(Move { from, to, promotion: None });
        }
    }
}


pub fn gen_bishops(board: &Board, moves: &mut MoveList) {
    let color = board.side_to_move;
    let bishops = board.pieces[color.index()][Piece::Bishop.index()];
    let own = board.occupied_by(color);
    let occ = board.occupied;

    let mut bb = bishops;
    while bb != 0 {
        let from = pop_lsb(&mut bb);
        let mut targets = bishop_attacks(from, occ) & !own;

        while targets != 0 {
            let to = pop_lsb(&mut targets);
            moves.push(Move { from, to, promotion: None });
        }
    }
}

pub fn gen_rooks(board: &Board, moves: &mut MoveList) {
    let color = board.side_to_move;
    let rooks = board.pieces[color.index()][Piece::Rook.index()];
    let own = board.occupied_by(color);
    let occ = board.occupied;

    let mut bb = rooks;
    while bb != 0 {
        let from = pop_lsb(&mut bb);
        let mut targets = rook_attacks(from, occ) & !own;

        while targets != 0 {
            let to = pop_lsb(&mut targets);
            moves.push(Move { from, to, promotion: None });
        }
    }
}

pub fn gen_queens(board: &Board, moves: &mut MoveList) {
    let color = board.side_to_move;
    let queens = board.pieces[color.index()][Piece::Queen.index()];
    let own = board.occupied_by(color);
    let occ = board.occupied;

    let mut bb = queens;
    while bb != 0 {
        let from = pop_lsb(&mut bb);
        let mut targets = queen_attacks(from, occ) & !own;

        while targets != 0 {
            let to = pop_lsb(&mut targets);
            moves.push(Move { from, to, promotion: None });
        }
    }
}

pub fn gen_king(board: &Board, moves: &mut MoveList) {
    let color = board.side_to_move;
    let king = board.pieces[color.index()][Piece::King.index()];
    let own = board.occupied_by(color);

    let from = king.trailing_zeros() as u8;
    let mut targets = king_attacks(from) & !own;

    while targets != 0 {
        let to = pop_lsb(&mut targets);
        moves.push(Move { from, to, promotion: None });
    }

    // Generate castling moves
    gen_castling(board, moves);
}

pub fn gen_castling(board: &Board, moves: &mut MoveList) {
    let color = board.side_to_move;
    let occ = board.occupied;
    let enemy = color.opposite();

    // Can't castle if in check
    if board.in_check(color) {
        return;
    }

    match color {
        Color::White => {
            // Kingside castling (e1g1)
            if (board.castling_rights & CASTLE_WHITE_KING) != 0 {
                // Check f1 and g1 are empty
                if (occ & (bb(5) | bb(6))) == 0 {
                    // Check that f1 and g1 are not attacked
                    if !board.is_square_attacked(5, enemy) && !board.is_square_attacked(6, enemy) {
                        moves.push(Move { from: 4, to: 6, promotion: None });
                    }
                }
            }
            // Queenside castling (e1c1)
            if (board.castling_rights & CASTLE_WHITE_QUEEN) != 0 {
                // Check b1, c1, d1 are empty
                if (occ & (bb(1) | bb(2) | bb(3))) == 0 {
                    // Check that c1 and d1 are not attacked
                    if !board.is_square_attacked(2, enemy) && !board.is_square_attacked(3, enemy) {
                        moves.push(Move { from: 4, to: 2, promotion: None });
                    }
                }
            }
        },
        Color::Black => {
            // Kingside castling (e8g8)
            if (board.castling_rights & CASTLE_BLACK_KING) != 0 {
                // Check f8 and g8 are empty
                if (occ & (bb(61) | bb(62))) == 0 {
                    // Check that f8 and g8 are not attacked
                    if !board.is_square_attacked(61, enemy) && !board.is_square_attacked(62, enemy) {
                        moves.push(Move { from: 60, to: 62, promotion: None });
                    }
                }
            }
            // Queenside castling (e8c8)
            if (board.castling_rights & CASTLE_BLACK_QUEEN) != 0 {
                // Check b8, c8, d8 are empty
                if (occ & (bb(57) | bb(58) | bb(59))) == 0 {
                    // Check that c8 and d8 are not attacked
                    if !board.is_square_attacked(58, enemy) && !board.is_square_attacked(59, enemy) {
                        moves.push(Move { from: 60, to: 58, promotion: None });
                    }
                }
            }
        }
    }
}


pub fn gen_pawns(board: &Board, moves: &mut MoveList) {
    let color = board.side_to_move;
    let pawns = board.pieces[color.index()][Piece::Pawn.index()];
    let occ = board.occupied;
    let enemy = board.occupied_by(color.opposite());

    match color {
        Color::White => {
            // Single pushes
            let single_push = (pawns << 8) & !occ;
            
            // Promotions (pushes to rank 8)
            let mut promo_pushes = single_push & RANK_8;
            while promo_pushes != 0 {
                let to = pop_lsb(&mut promo_pushes);
                let from = to - 8;
                for promo in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                    moves.push(Move { from, to, promotion: Some(promo.index() as u8) });
                }
            }
            
            // Non-promotion pushes
            let mut targets = single_push & !RANK_8;
            while targets != 0 {
                let to = pop_lsb(&mut targets);
                moves.push(Move {
                    from: to - 8,
                    to,
                    promotion: None,
                });
            }

            // Double pushes from rank 2
            let double_push = ((single_push & RANK_3) << 8) & !occ;
            let mut dbl = double_push;
            while dbl != 0 {
                let to = pop_lsb(&mut dbl);
                moves.push(Move {
                    from: to - 16,
                    to,
                    promotion: None,
                });
            }

            // Captures
            let left_caps = (pawns << 7) & enemy & !FILE_H;
            let right_caps = (pawns << 9) & enemy & !FILE_A;

            // Promotion captures - left
            let mut promo_left = left_caps & RANK_8;
            while promo_left != 0 {
                let to = pop_lsb(&mut promo_left);
                let from = to - 7;
                for promo in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                    moves.push(Move { from, to, promotion: Some(promo.index() as u8) });
                }
            }
            
            // Promotion captures - right
            let mut promo_right = right_caps & RANK_8;
            while promo_right != 0 {
                let to = pop_lsb(&mut promo_right);
                let from = to - 9;
                for promo in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                    moves.push(Move { from, to, promotion: Some(promo.index() as u8) });
                }
            }

            // Non-promotion captures - left
            let mut left = left_caps & !RANK_8;
            while left != 0 {
                let to = pop_lsb(&mut left);
                let from = to - 7;
                moves.push(Move { from, to, promotion: None });
            }
            
            // Non-promotion captures - right
            let mut right = right_caps & !RANK_8;
            while right != 0 {
                let to = pop_lsb(&mut right);
                let from = to - 9;
                moves.push(Move { from, to, promotion: None });
            }

            // En passant captures
            if let Some(ep_sq) = board.en_passant_square {
                // Check if a pawn can capture from the left (file-1)
                if ep_sq % 8 > 0 { // ep square not on a-file
                    let from = ep_sq - 9; // one rank down, one file left
                    if pawns & bb(from) != 0 {
                        moves.push(Move { from, to: ep_sq, promotion: None });
                    }
                }
                // Check if a pawn can capture from the right (file+1)
                if ep_sq % 8 < 7 { // ep square not on h-file
                    let from = ep_sq - 7; // one rank down, one file right
                    if pawns & bb(from) != 0 {
                        moves.push(Move { from, to: ep_sq, promotion: None });
                    }
                }
            }
        }

        Color::Black => {
            // Single pushes
            let single_push = (pawns >> 8) & !occ;
            
            // Promotions (pushes to rank 1)
            let mut promo_pushes = single_push & RANK_1;
            while promo_pushes != 0 {
                let to = pop_lsb(&mut promo_pushes);
                let from = to + 8;
                for promo in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                    moves.push(Move { from, to, promotion: Some(promo.index() as u8) });
                }
            }
            
            // Non-promotion pushes
            let mut targets = single_push & !RANK_1;
            while targets != 0 {
                let to = pop_lsb(&mut targets);
                moves.push(Move {
                    from: to + 8,
                    to,
                    promotion: None,
                });
            }

            // Double pushes from rank 7
            let double_push = ((single_push & RANK_6) >> 8) & !occ;
            let mut dbl = double_push;
            while dbl != 0 {
                let to = pop_lsb(&mut dbl);
                moves.push(Move {
                    from: to + 16,
                    to,
                    promotion: None,
                });
            }

            // Captures
            let left_caps = (pawns >> 9) & enemy & !FILE_H;
            let right_caps = (pawns >> 7) & enemy & !FILE_A;

            // Promotion captures - left
            let mut promo_left = left_caps & RANK_1;
            while promo_left != 0 {
                let to = pop_lsb(&mut promo_left);
                let from = to + 9;
                for promo in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                    moves.push(Move { from, to, promotion: Some(promo.index() as u8) });
                }
            }
            
            // Promotion captures - right
            let mut promo_right = right_caps & RANK_1;
            while promo_right != 0 {
                let to = pop_lsb(&mut promo_right);
                let from = to + 7;
                for promo in [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight] {
                    moves.push(Move { from, to, promotion: Some(promo.index() as u8) });
                }
            }

            // Non-promotion captures - left
            let mut left = left_caps & !RANK_1;
            while left != 0 {
                let to = pop_lsb(&mut left);
                let from = to + 9;
                moves.push(Move { from, to, promotion: None });
            }
            
            // Non-promotion captures - right
            let mut right = right_caps & !RANK_1;
            while right != 0 {
                let to = pop_lsb(&mut right);
                let from = to + 7;
                moves.push(Move { from, to, promotion: None });
            }

            // En passant captures
            if let Some(ep_sq) = board.en_passant_square {
                // Check if a pawn can capture from the left (file-1)
                if ep_sq % 8 > 0 { // ep square not on a-file
                    let from = ep_sq + 7; // one rank up, one file left
                    if pawns & bb(from) != 0 {
                        moves.push(Move { from, to: ep_sq, promotion: None });
                    }
                }
                // Check if a pawn can capture from the right (file+1)
                if ep_sq % 8 < 7 { // ep square not on h-file
                    let from = ep_sq + 9; // one rank up, one file right
                    if pawns & bb(from) != 0 {
                        moves.push(Move { from, to: ep_sq, promotion: None });
                    }
                }
            }
        }
    }
}

pub fn generate_moves(board: &Board) -> MoveList {
    let mut moves = Vec::with_capacity(64);

    gen_pawns(board, &mut moves);
    gen_knights(board, &mut moves);
    gen_bishops(board, &mut moves);
    gen_rooks(board, &mut moves);
    gen_queens(board, &mut moves);
    gen_king(board, &mut moves);

    moves
}

pub fn generate_legal_moves(board: &mut Board) -> MoveList {
    let color = board.side_to_move;
    let pseudo_moves = generate_moves(board);
    let mut legal_moves = Vec::with_capacity(pseudo_moves.len());

    for mv in pseudo_moves {
        let undo = board.make_move(mv);
        if !board.in_check(color) {
            legal_moves.push(mv);
        }
        board.unmake_move(mv, undo);
    }

    legal_moves
}