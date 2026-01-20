use crate::board::bitboard::*;
use crate::board::piece::{Piece, Color, index_to_piece};
use crate::board::undo::Undo;
use crate::board::r#move::Move;
use crate::movegen::attacks::{
    rook_attacks,
    bishop_attacks,
    knight_attacks,
    king_attacks,
};


#[derive(Clone)]
pub struct Board {
    pub pieces: [[Bitboard; 6]; 2], // [color][piece]
    pub side_to_move: Color,
    pub occupied: Bitboard,
    pub castling_rights: u8,
    pub en_passant_square: Option<u8>,
}

impl Piece {
    #[inline]
    pub fn index(self) -> usize {
        match self {
            Piece::Pawn => 0,
            Piece::Knight => 1,
            Piece::Bishop => 2,
            Piece::Rook => 3,
            Piece::Queen => 4,
            Piece::King => 5,
        }
    }
}

impl Color {
    #[inline]
    pub fn index(self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 1,
        }
    }
}

impl Board {
    pub fn empty() -> Self {
        Board {
            pieces: [[0; 6]; 2],
            side_to_move: Color::White,
            occupied: 0,
            castling_rights: 0,
            en_passant_square: None,
        }
    }

    pub fn startpos() -> Self {
        let mut b = Board::empty();

        // Pawns
        b.pieces[0][0] = RANK_2;
        b.pieces[1][0] = RANK_7;

        // Rooks
        b.pieces[0][3] = bb(0) | bb(7);
        b.pieces[1][3] = bb(56) | bb(63);

        // Knights
        b.pieces[0][1] = bb(1) | bb(6);
        b.pieces[1][1] = bb(57) | bb(62);

        // Bishops
        b.pieces[0][2] = bb(2) | bb(5);
        b.pieces[1][2] = bb(58) | bb(61);

        // Queens
        b.pieces[0][4] = bb(3);
        b.pieces[1][4] = bb(59);

        // Kings
        b.pieces[0][5] = bb(4);
        b.pieces[1][5] = bb(60);

        b.castling_rights = CASTLE_WHITE_KING | CASTLE_WHITE_QUEEN | CASTLE_BLACK_KING | CASTLE_BLACK_QUEEN;
        b.recompute_occupancy();
        b
    }

    #[inline]
    pub fn recompute_occupancy(&mut self) {
        self.occupied = 0;
        for c in 0..2 {
            for p in 0..6 {
                self.occupied |= self.pieces[c][p];
            }
        }
    }

    #[inline]
    pub fn occupied_by(&self, color: Color) -> Bitboard {
        let mut occ = 0;
        for p in 0..6 {
            occ |= self.pieces[color.index()][p];
        }
        occ
    }

    #[inline]
    pub fn piece_at(&self, square: u8) -> Option<(Color, Piece)> {
        let mask = bb(square);
        for &color in &[Color::White, Color::Black] {
            for &piece in &[
                Piece::Pawn,
                Piece::Knight,
                Piece::Bishop,
                Piece::Rook,
                Piece::Queen,
                Piece::King,
            ] {
                if self.pieces[color.index()][piece.index()] & mask != 0 {
                    return Some((color, piece));
                }
            }
        }
        None
    }

    pub fn make_move(&mut self, mv: Move) -> Undo {
        let from_mask = bb(mv.from);
        let to_mask = bb(mv.to);

        let color = self.side_to_move;
        let enemy = color.opposite();

        let mut moved_piece = None;

        // Find moving piece
        for p in 0..6 {
            if self.pieces[color.index()][p] & from_mask != 0 {
                self.pieces[color.index()][p] ^= from_mask;
                self.pieces[color.index()][p] |= to_mask;
                moved_piece = Some(p);
                break;
            }
        }

        debug_assert!(moved_piece.is_some());

        // Handle capture
        let mut captured = None;
        for p in 0..6 {
            if self.pieces[enemy.index()][p] & to_mask != 0 {
                self.pieces[enemy.index()][p] ^= to_mask;
                captured = Some((enemy, index_to_piece(p)));
                break;
            }
        }

        // Handle en passant capture
        if moved_piece == Some(Piece::Pawn.index()) && Some(mv.to) == self.en_passant_square {
            // Remove the captured pawn (it's not on the 'to' square)
            let captured_pawn_sq = match color {
                Color::White => mv.to - 8,
                Color::Black => mv.to + 8,
            };
            self.pieces[enemy.index()][Piece::Pawn.index()] ^= bb(captured_pawn_sq);
            captured = Some((enemy, Piece::Pawn));
        }

        // Handle promotion
        if let Some(promo_piece) = mv.promotion {
            // Remove the pawn from destination and add the promoted piece
            self.pieces[color.index()][Piece::Pawn.index()] ^= to_mask;
            self.pieces[color.index()][promo_piece as usize] |= to_mask;
        }

        // Update occupancy
        self.recompute_occupancy();

        let undo = Undo {
            captured,
            side_to_move: self.side_to_move,
            castling_rights: self.castling_rights,
            en_passant_square: self.en_passant_square,
        };

        // Update en passant square
        self.en_passant_square = None;
        if moved_piece == Some(Piece::Pawn.index()) && mv.from.abs_diff(mv.to) == 16 {
            // Pawn moved two squares, set en passant square
            self.en_passant_square = Some((mv.from + mv.to) / 2);
        }

        // Handle castling rook movement
        if moved_piece == Some(Piece::King.index()) && mv.from.abs_diff(mv.to) == 2 {
            match (color, mv.to) {
                (Color::White, 6) => { // Kingside castle (e1g1)
                    self.pieces[0][Piece::Rook.index()] ^= bb(7); // Remove from h1
                    self.pieces[0][Piece::Rook.index()] |= bb(5); // Add to f1
                },
                (Color::White, 2) => { // Queenside castle (e1c1)
                    self.pieces[0][Piece::Rook.index()] ^= bb(0); // Remove from a1
                    self.pieces[0][Piece::Rook.index()] |= bb(3); // Add to d1
                },
                (Color::Black, 62) => { // Kingside castle (e8g8)
                    self.pieces[1][Piece::Rook.index()] ^= bb(63); // Remove from h8
                    self.pieces[1][Piece::Rook.index()] |= bb(61); // Add to f8
                },
                (Color::Black, 58) => { // Queenside castle (e8c8)
                    self.pieces[1][Piece::Rook.index()] ^= bb(56); // Remove from a8
                    self.pieces[1][Piece::Rook.index()] |= bb(59); // Add to d8
                },
                _ => {}
            }
            self.recompute_occupancy();
        }

        // Update castling rights
        if moved_piece == Some(Piece::King.index()) {
            if color == Color::White {
                self.castling_rights &= !(CASTLE_WHITE_KING | CASTLE_WHITE_QUEEN);
            } else {
                self.castling_rights &= !(CASTLE_BLACK_KING | CASTLE_BLACK_QUEEN);
            }
        } else if moved_piece == Some(Piece::Rook.index()) {
            match mv.from {
                0 => self.castling_rights &= !CASTLE_WHITE_QUEEN,
                7 => self.castling_rights &= !CASTLE_WHITE_KING,
                56 => self.castling_rights &= !CASTLE_BLACK_QUEEN,
                63 => self.castling_rights &= !CASTLE_BLACK_KING,
                _ => {}
            }
        }
        // If a rook is captured, remove castling rights
        if captured.is_some() {
            match mv.to {
                0 => self.castling_rights &= !CASTLE_WHITE_QUEEN,
                7 => self.castling_rights &= !CASTLE_WHITE_KING,
                56 => self.castling_rights &= !CASTLE_BLACK_QUEEN,
                63 => self.castling_rights &= !CASTLE_BLACK_KING,
                _ => {}
            }
        }

        self.side_to_move = enemy;

        undo
    }

    pub fn unmake_move(&mut self, mv: Move, undo: Undo) {
        let from_mask = bb(mv.from);
        let to_mask = bb(mv.to);

        let color = undo.side_to_move;

        // Restore side to move, castling rights, and en passant
        self.side_to_move = color;
        self.castling_rights = undo.castling_rights;
        self.en_passant_square = undo.en_passant_square;

        // Check if this was a castling move
        let is_castling = mv.from.abs_diff(mv.to) == 2 && 
                          self.pieces[color.index()][Piece::King.index()] & to_mask != 0;

        if is_castling {
            // Move king back
            self.pieces[color.index()][Piece::King.index()] ^= to_mask;
            self.pieces[color.index()][Piece::King.index()] |= from_mask;

            // Move rook back
            match (color, mv.to) {
                (Color::White, 6) => { // Kingside castle
                    self.pieces[0][Piece::Rook.index()] ^= bb(5); // Remove from f1
                    self.pieces[0][Piece::Rook.index()] |= bb(7); // Add to h1
                },
                (Color::White, 2) => { // Queenside castle
                    self.pieces[0][Piece::Rook.index()] ^= bb(3); // Remove from d1
                    self.pieces[0][Piece::Rook.index()] |= bb(0); // Add to a1
                },
                (Color::Black, 62) => { // Kingside castle
                    self.pieces[1][Piece::Rook.index()] ^= bb(61); // Remove from f8
                    self.pieces[1][Piece::Rook.index()] |= bb(63); // Add to h8
                },
                (Color::Black, 58) => { // Queenside castle
                    self.pieces[1][Piece::Rook.index()] ^= bb(59); // Remove from d8
                    self.pieces[1][Piece::Rook.index()] |= bb(56); // Add to a8
                },
                _ => {}
            }
        } else {
            // Handle promotion undo
            if let Some(promo_piece) = mv.promotion {
                // Remove the promoted piece from destination
                self.pieces[color.index()][promo_piece as usize] ^= to_mask;
                // Add the pawn back to the from square
                self.pieces[color.index()][Piece::Pawn.index()] |= from_mask;
            } else {
                // Move piece back
                for p in 0..6 {
                    if self.pieces[color.index()][p] & to_mask != 0 {
                        self.pieces[color.index()][p] ^= to_mask;
                        self.pieces[color.index()][p] |= from_mask;
                        break;
                    }
                }
            }

            // Check if this was an en passant capture
            // En passant is when: pawn captured, and the to square was the en passant square
            let was_en_passant = self.pieces[color.index()][Piece::Pawn.index()] & from_mask != 0 
                                 && undo.captured.is_some() 
                                 && undo.captured.unwrap().1 == Piece::Pawn 
                                 && undo.en_passant_square == Some(mv.to);

            // Restore captured piece
            if let Some((c, piece)) = undo.captured {
                if was_en_passant {
                    // Restore the pawn to the correct square (not the 'to' square)
                    let captured_pawn_sq = match color {
                        Color::White => mv.to - 8,
                        Color::Black => mv.to + 8,
                    };
                    self.pieces[c.index()][piece.index()] |= bb(captured_pawn_sq);
                } else {
                    self.pieces[c.index()][piece.index()] |= to_mask;
                }
            }
        }

        self.recompute_occupancy();
    }

    pub fn is_square_attacked(&self, sq: u8, by: Color) -> bool {
        let attackers = self.pieces[by.index()];
        let occ = self.occupied;
        let sq_bb = bb(sq);

        // ───── Pawns ─────
        match by {
            Color::White => {
                let pawns = attackers[Piece::Pawn.index()];
                let attacks =
                    ((pawns << 7) & !FILE_H) |
                    ((pawns << 9) & !FILE_A);
                if attacks & sq_bb != 0 {
                    return true;
                }
            }
            Color::Black => {
                let pawns = attackers[Piece::Pawn.index()];
                let attacks =
                    ((pawns >> 7) & !FILE_A) |
                    ((pawns >> 9) & !FILE_H);
                if attacks & sq_bb != 0 {
                    return true;
                }
            }
        }

        // ───── Knights ─────
        let knights = attackers[Piece::Knight.index()];
        if knight_attacks(sq) & knights != 0 {
            return true;
        }

        // ───── Bishops / Queens ─────
        let bishops = attackers[Piece::Bishop.index()];
        let queens = attackers[Piece::Queen.index()];
        if bishop_attacks(sq, occ) & (bishops | queens) != 0 {
            return true;
        }

        // ───── Rooks / Queens ─────
        let rooks = attackers[Piece::Rook.index()];
        if rook_attacks(sq, occ) & (rooks | queens) != 0 {
            return true;
        }

        // ───── King ─────
        let king = attackers[Piece::King.index()];
        if king_attacks(sq) & king != 0 {
            return true;
        }

        false
    }

    pub fn in_check(&self, color: Color) -> bool {
        let king_bb = self.pieces[color.index()][Piece::King.index()];
        debug_assert!(king_bb != 0);

        let king_sq = king_bb.trailing_zeros() as u8;
        self.is_square_attacked(king_sq, color.opposite())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::bitboard::Square;

    #[test]
    fn startpos_piece_count() {
        let b = Board::startpos();
        assert_eq!(b.occupied.count_ones(), 32);
    }

    #[test]
    fn kings_in_correct_place() {
        let b = Board::startpos();
        assert_eq!(
            b.piece_at(Square::E1 as u8),
            Some((Color::White, Piece::King))
        );
        assert_eq!(
            b.piece_at(Square::E8 as u8),
            Some((Color::Black, Piece::King))
        );
    }
}

#[cfg(test)]
mod make_unmake_tests {
    use super::*;
    use crate::board::bitboard::Square;
    use crate::board::r#move::Move;
    use crate::board::piece::{Color, Piece};

    #[test]
    fn make_and_unmake_quiet_move() {
        let mut board = Board::startpos();

        let mv = Move {
            from: Square::G1 as u8,
            to: Square::F3 as u8,
            promotion: None,
        };

        let undo = board.make_move(mv);

        assert!(board.piece_at(Square::F3 as u8).is_some());
        assert!(board.piece_at(Square::G1 as u8).is_none());

        board.unmake_move(mv, undo);

        assert!(board.piece_at(Square::G1 as u8).is_some());
        assert!(board.piece_at(Square::F3 as u8).is_none());
    }

    #[test]
    fn make_and_unmake_capture() {
        let mut board = Board::empty();

        board.pieces[0][Piece::Knight.index()] = bb(Square::E4 as u8);
        board.pieces[1][Piece::Pawn.index()] = bb(Square::D6 as u8);
        board.recompute_occupancy();

        let mv = Move {
            from: Square::E4 as u8,
            to: Square::D6 as u8,
            promotion: None,
        };

        let undo = board.make_move(mv);
        assert!(board.piece_at(Square::D6 as u8).unwrap().0 == Color::White);

        board.unmake_move(mv, undo);
        assert!(board.piece_at(Square::E4 as u8).is_some());
        assert!(board.piece_at(Square::D6 as u8).unwrap().0 == Color::Black);
    }

    #[test]
    fn board_restored_exactly() {
        let mut board = Board::startpos();
        let original = board.clone();

        let mv = Move {
            from: Square::B1 as u8,
            to: Square::C3 as u8,
            promotion: None,
        };

        let undo = board.make_move(mv);
        board.unmake_move(mv, undo);

        assert_eq!(board.occupied, original.occupied);
        assert_eq!(board.side_to_move, original.side_to_move);
    }
}

#[cfg(test)]
mod check_tests {
    use super::*;
    use crate::board::bitboard::Square;
    use crate::board::piece::{Color, Piece};

    #[test]
    fn knight_check() {
        let mut board = Board::empty();
        board.pieces[Color::White.index()][Piece::King.index()] = bb(Square::E4 as u8);
        board.pieces[Color::Black.index()][Piece::Knight.index()] = bb(Square::F6 as u8);
        board.recompute_occupancy();

        assert!(board.in_check(Color::White));
    }

    #[test]
    fn bishop_check_blocked() {
        let mut board = Board::empty();
        board.pieces[0][Piece::King.index()] = bb(Square::E4 as u8);
        board.pieces[1][Piece::Bishop.index()] = bb(Square::A8 as u8);
        board.pieces[0][Piece::Pawn.index()] = bb(Square::C6 as u8);
        board.recompute_occupancy();

        assert!(!board.in_check(Color::White));
    }

    #[test]
    fn rook_check() {
        let mut board = Board::empty();
        board.pieces[0][Piece::King.index()] = bb(Square::E1 as u8);
        board.pieces[1][Piece::Rook.index()] = bb(Square::E8 as u8);
        board.recompute_occupancy();

        assert!(board.in_check(Color::White));
    }

    #[test]
    fn pawn_check() {
        let mut board = Board::empty();
        board.pieces[0][Piece::King.index()] = bb(Square::E4 as u8);
        board.pieces[1][Piece::Pawn.index()] = bb(Square::D5 as u8);
        board.recompute_occupancy();

        assert!(board.in_check(Color::White));
    }

    #[test]
    fn king_adjacent_check() {
        let mut board = Board::empty();
        board.pieces[0][Piece::King.index()] = bb(Square::E4 as u8);
        board.pieces[1][Piece::King.index()] = bb(Square::E5 as u8);
        board.recompute_occupancy();

        assert!(board.in_check(Color::White));
    }
}

#[cfg(test)]
mod en_passant_tests {
    use super::*;
    use crate::board::bitboard::Square;
    use crate::board::piece::{Color, Piece};
    use crate::board::r#move::Move;

    #[test]
    fn white_pawn_double_push_sets_en_passant() {
        let mut board = Board::startpos();
        
        let mv = Move {
            from: Square::E2 as u8,
            to: Square::E4 as u8,
            promotion: None,
        };
        
        board.make_move(mv);
        
        assert_eq!(board.en_passant_square, Some(Square::E3 as u8));
    }

    #[test]
    fn black_pawn_double_push_sets_en_passant() {
        let mut board = Board::startpos();
        board.side_to_move = Color::Black;
        
        let mv = Move {
            from: Square::D7 as u8,
            to: Square::D5 as u8,
            promotion: None,
        };
        
        board.make_move(mv);
        
        assert_eq!(board.en_passant_square, Some(Square::D6 as u8));
    }

    #[test]
    fn white_captures_en_passant() {
        let mut board = Board::empty();
        board.pieces[Color::White.index()][Piece::Pawn.index()] = bb(Square::E5 as u8);
        board.pieces[Color::Black.index()][Piece::Pawn.index()] = bb(Square::D5 as u8);
        board.en_passant_square = Some(Square::D6 as u8);
        board.recompute_occupancy();
        
        let mv = Move {
            from: Square::E5 as u8,
            to: Square::D6 as u8,
            promotion: None,
        };
        
        board.make_move(mv);
        
        // White pawn should be on d6
        assert!(board.pieces[Color::White.index()][Piece::Pawn.index()] & bb(Square::D6 as u8) != 0);
        // Black pawn on d5 should be captured
        assert!(board.pieces[Color::Black.index()][Piece::Pawn.index()] & bb(Square::D5 as u8) == 0);
        // Original square should be empty
        assert!(board.pieces[Color::White.index()][Piece::Pawn.index()] & bb(Square::E5 as u8) == 0);
    }

    #[test]
    fn black_captures_en_passant() {
        let mut board = Board::empty();
        board.pieces[Color::Black.index()][Piece::Pawn.index()] = bb(Square::F4 as u8);
        board.pieces[Color::White.index()][Piece::Pawn.index()] = bb(Square::E4 as u8);
        board.en_passant_square = Some(Square::E3 as u8);
        board.side_to_move = Color::Black;
        board.recompute_occupancy();
        
        let mv = Move {
            from: Square::F4 as u8,
            to: Square::E3 as u8,
            promotion: None,
        };
        
        board.make_move(mv);
        
        // Black pawn should be on e3
        assert!(board.pieces[Color::Black.index()][Piece::Pawn.index()] & bb(Square::E3 as u8) != 0);
        // White pawn on e4 should be captured
        assert!(board.pieces[Color::White.index()][Piece::Pawn.index()] & bb(Square::E4 as u8) == 0);
        // Original square should be empty
        assert!(board.pieces[Color::Black.index()][Piece::Pawn.index()] & bb(Square::F4 as u8) == 0);
    }

    #[test]
    fn en_passant_cleared_after_other_move() {
        let mut board = Board::startpos();
        
        // White pawn double push
        let mv1 = Move {
            from: Square::E2 as u8,
            to: Square::E4 as u8,
            promotion: None,
        };
        board.make_move(mv1);
        assert_eq!(board.en_passant_square, Some(Square::E3 as u8));
        
        // Black makes a different move
        let mv2 = Move {
            from: Square::B8 as u8,
            to: Square::C6 as u8,
            promotion: None,
        };
        board.make_move(mv2);
        
        // En passant should be cleared
        assert_eq!(board.en_passant_square, None);
    }

    #[test]
    fn unmake_en_passant_capture() {
        let mut board = Board::empty();
        board.pieces[Color::White.index()][Piece::Pawn.index()] = bb(Square::E5 as u8);
        board.pieces[Color::Black.index()][Piece::Pawn.index()] = bb(Square::D5 as u8);
        board.en_passant_square = Some(Square::D6 as u8);
        board.recompute_occupancy();
        
        let original = board.clone();
        
        let mv = Move {
            from: Square::E5 as u8,
            to: Square::D6 as u8,
            promotion: None,
        };
        
        let undo = board.make_move(mv);
        board.unmake_move(mv, undo);
        
        // Board should be restored exactly
        assert_eq!(board.pieces[Color::White.index()][Piece::Pawn.index()], 
                   original.pieces[Color::White.index()][Piece::Pawn.index()]);
        assert_eq!(board.pieces[Color::Black.index()][Piece::Pawn.index()], 
                   original.pieces[Color::Black.index()][Piece::Pawn.index()]);
        assert_eq!(board.en_passant_square, original.en_passant_square);
    }
}

#[cfg(test)]
mod castling_tests {
    use super::*;
    use crate::board::bitboard::Square;
    use crate::board::piece::{Color, Piece};
    use crate::board::r#move::Move;

    #[test]
    fn white_kingside_castle_moves_rook() {
        let mut board = Board::empty();
        board.pieces[Color::White.index()][Piece::King.index()] = bb(Square::E1 as u8);
        board.pieces[Color::White.index()][Piece::Rook.index()] = bb(Square::H1 as u8);
        board.castling_rights = CASTLE_WHITE_KING;
        board.recompute_occupancy();
        
        let mv = Move {
            from: Square::E1 as u8,
            to: Square::G1 as u8,
            promotion: None,
        };
        
        board.make_move(mv);
        
        // King should be on g1
        assert!(board.pieces[Color::White.index()][Piece::King.index()] & bb(Square::G1 as u8) != 0);
        // Rook should be on f1
        assert!(board.pieces[Color::White.index()][Piece::Rook.index()] & bb(Square::F1 as u8) != 0);
        // Rook should not be on h1
        assert!(board.pieces[Color::White.index()][Piece::Rook.index()] & bb(Square::H1 as u8) == 0);
    }

    #[test]
    fn white_queenside_castle_moves_rook() {
        let mut board = Board::empty();
        board.pieces[Color::White.index()][Piece::King.index()] = bb(Square::E1 as u8);
        board.pieces[Color::White.index()][Piece::Rook.index()] = bb(Square::A1 as u8);
        board.castling_rights = CASTLE_WHITE_QUEEN;
        board.recompute_occupancy();
        
        let mv = Move {
            from: Square::E1 as u8,
            to: Square::C1 as u8,
            promotion: None,
        };
        
        board.make_move(mv);
        
        // King should be on c1
        assert!(board.pieces[Color::White.index()][Piece::King.index()] & bb(Square::C1 as u8) != 0);
        // Rook should be on d1
        assert!(board.pieces[Color::White.index()][Piece::Rook.index()] & bb(Square::D1 as u8) != 0);
        // Rook should not be on a1
        assert!(board.pieces[Color::White.index()][Piece::Rook.index()] & bb(Square::A1 as u8) == 0);
    }

    #[test]
    fn black_kingside_castle_moves_rook() {
        let mut board = Board::empty();
        board.pieces[Color::Black.index()][Piece::King.index()] = bb(Square::E8 as u8);
        board.pieces[Color::Black.index()][Piece::Rook.index()] = bb(Square::H8 as u8);
        board.castling_rights = CASTLE_BLACK_KING;
        board.side_to_move = Color::Black;
        board.recompute_occupancy();
        
        let mv = Move {
            from: Square::E8 as u8,
            to: Square::G8 as u8,
            promotion: None,
        };
        
        board.make_move(mv);
        
        // King should be on g8
        assert!(board.pieces[Color::Black.index()][Piece::King.index()] & bb(Square::G8 as u8) != 0);
        // Rook should be on f8
        assert!(board.pieces[Color::Black.index()][Piece::Rook.index()] & bb(Square::F8 as u8) != 0);
        // Rook should not be on h8
        assert!(board.pieces[Color::Black.index()][Piece::Rook.index()] & bb(Square::H8 as u8) == 0);
    }

    #[test]
    fn black_queenside_castle_moves_rook() {
        let mut board = Board::empty();
        board.pieces[Color::Black.index()][Piece::King.index()] = bb(Square::E8 as u8);
        board.pieces[Color::Black.index()][Piece::Rook.index()] = bb(Square::A8 as u8);
        board.castling_rights = CASTLE_BLACK_QUEEN;
        board.side_to_move = Color::Black;
        board.recompute_occupancy();
        
        let mv = Move {
            from: Square::E8 as u8,
            to: Square::C8 as u8,
            promotion: None,
        };
        
        board.make_move(mv);
        
        // King should be on c8
        assert!(board.pieces[Color::Black.index()][Piece::King.index()] & bb(Square::C8 as u8) != 0);
        // Rook should be on d8
        assert!(board.pieces[Color::Black.index()][Piece::Rook.index()] & bb(Square::D8 as u8) != 0);
        // Rook should not be on a8
        assert!(board.pieces[Color::Black.index()][Piece::Rook.index()] & bb(Square::A8 as u8) == 0);
    }

    #[test]
    fn castling_rights_removed_after_king_move() {
        let mut board = Board::startpos();
        
        let mv = Move {
            from: Square::E1 as u8,
            to: Square::E2 as u8,
            promotion: None,
        };
        
        board.make_move(mv);
        
        // White should lose both castling rights
        assert_eq!(board.castling_rights & (CASTLE_WHITE_KING | CASTLE_WHITE_QUEEN), 0);
        // Black should still have castling rights
        assert_ne!(board.castling_rights & (CASTLE_BLACK_KING | CASTLE_BLACK_QUEEN), 0);
    }

    #[test]
    fn castling_rights_removed_after_rook_move() {
        let mut board = Board::startpos();
        
        // Move h1 rook
        let mv = Move {
            from: Square::H1 as u8,
            to: Square::H2 as u8,
            promotion: None,
        };
        
        board.make_move(mv);
        
        // White should lose kingside castling only
        assert_eq!(board.castling_rights & CASTLE_WHITE_KING, 0);
        assert_ne!(board.castling_rights & CASTLE_WHITE_QUEEN, 0);
    }

    #[test]
    fn castling_rights_removed_when_rook_captured() {
        let mut board = Board::empty();
        board.pieces[Color::White.index()][Piece::Rook.index()] = bb(Square::A1 as u8) | bb(Square::H1 as u8);
        board.pieces[Color::White.index()][Piece::King.index()] = bb(Square::E1 as u8);
        board.pieces[Color::Black.index()][Piece::Bishop.index()] = bb(Square::C3 as u8);
        board.pieces[Color::Black.index()][Piece::King.index()] = bb(Square::E8 as u8);
        board.castling_rights = CASTLE_WHITE_KING | CASTLE_WHITE_QUEEN;
        board.side_to_move = Color::Black;
        board.recompute_occupancy();
        
        // Black bishop captures a1 rook
        let mv = Move {
            from: Square::C3 as u8,
            to: Square::A1 as u8,
            promotion: None,
        };
        
        board.make_move(mv);
        
        // White should lose queenside castling
        assert_eq!(board.castling_rights & CASTLE_WHITE_QUEEN, 0);
        // But still have kingside
        assert_ne!(board.castling_rights & CASTLE_WHITE_KING, 0);
    }

    #[test]
    fn unmake_castle_restores_pieces() {
        let mut board = Board::empty();
        board.pieces[Color::White.index()][Piece::King.index()] = bb(Square::E1 as u8);
        board.pieces[Color::White.index()][Piece::Rook.index()] = bb(Square::H1 as u8);
        board.castling_rights = CASTLE_WHITE_KING;
        board.recompute_occupancy();
        
        let original = board.clone();
        
        let mv = Move {
            from: Square::E1 as u8,
            to: Square::G1 as u8,
            promotion: None,
        };
        
        let undo = board.make_move(mv);
        board.unmake_move(mv, undo);
        
        // Board should be restored exactly
        assert_eq!(board.pieces[Color::White.index()][Piece::King.index()], 
                   original.pieces[Color::White.index()][Piece::King.index()]);
        assert_eq!(board.pieces[Color::White.index()][Piece::Rook.index()], 
                   original.pieces[Color::White.index()][Piece::Rook.index()]);
        assert_eq!(board.castling_rights, original.castling_rights);
    }

    #[test]
    fn castling_rights_restored_on_unmake() {
        let mut board = Board::startpos();
        let original_rights = board.castling_rights;
        
        let mv = Move {
            from: Square::E1 as u8,
            to: Square::E2 as u8,
            promotion: None,
        };
        
        let undo = board.make_move(mv);
        board.unmake_move(mv, undo);
        
        assert_eq!(board.castling_rights, original_rights);
    }
}