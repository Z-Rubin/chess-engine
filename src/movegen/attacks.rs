use crate::board::bitboard::{Bitboard, bb, FILE_A, FILE_H};

pub fn knight_attacks(square: u8) -> Bitboard {
    let b = bb(square);
    let mut attacks = 0;

    attacks |= (b << 17) & !FILE_A;
    attacks |= (b << 15) & !FILE_H;
    attacks |= (b << 10) & !(FILE_A | (FILE_A << 1));
    attacks |= (b << 6)  & !(FILE_H | (FILE_H >> 1));

    attacks |= (b >> 17) & !FILE_H;
    attacks |= (b >> 15) & !FILE_A;
    attacks |= (b >> 10) & !(FILE_H | (FILE_H >> 1));
    attacks |= (b >> 6)  & !(FILE_A | (FILE_A << 1));

    attacks
}

pub fn king_attacks(square: u8) -> Bitboard {
    let b = bb(square);
    let mut attacks = 0;

    attacks |= b << 8; // up
    attacks |= b >> 8; // down
    attacks |= (b << 1) & !FILE_A; // right
    attacks |= (b >> 1) & !FILE_H; // left
    attacks |= (b << 9) & !FILE_A; // up-right
    attacks |= (b << 7) & !FILE_H; // up-left
    attacks |= (b >> 7) & !FILE_A; // down-right
    attacks |= (b >> 9) & !FILE_H; // down-left

    attacks
}

/// White pawn attacks
pub fn white_pawn_attacks(square: u8) -> Bitboard {
    let b = bb(square);
    ((b << 7) & !FILE_H) | ((b << 9) & !FILE_A)
}

/// Black pawn attacks
pub fn black_pawn_attacks(square: u8) -> Bitboard {
    let b = bb(square);
    ((b >> 7) & !FILE_A) | ((b >> 9) & !FILE_H)
}

#[inline]
fn sliding_ray(
    square: u8,
    occupied: Bitboard,
    delta: i8,
    stop_file: fn(u8) -> bool,
) -> Bitboard {
    let mut attacks = 0;
    let mut s = square as i8;

    loop {
        // Check if we're at an edge before moving
        if stop_file(s as u8) {
            break;
        }

        s += delta;
        if s < 0 || s >= 64 {
            break;
        }

        let sq = s as u8;

        let bb_sq = bb(sq);
        attacks |= bb_sq;

        if occupied & bb_sq != 0 {
            break; // blocker hit
        }
    }

    attacks
}

#[inline] fn on_file_a(sq: u8) -> bool { sq % 8 == 0 }
#[inline] fn on_file_h(sq: u8) -> bool { sq % 8 == 7 }

pub fn rook_attacks(square: u8, occupied: Bitboard) -> Bitboard {
    let mut attacks = 0;

    attacks |= sliding_ray(square, occupied,  8, |_| false);      // north
    attacks |= sliding_ray(square, occupied, -8, |_| false);      // south
    attacks |= sliding_ray(square, occupied,  1, on_file_h);      // east
    attacks |= sliding_ray(square, occupied, -1, on_file_a);      // west

    attacks
}

pub fn bishop_attacks(square: u8, occupied: Bitboard) -> Bitboard {
    let mut attacks = 0;

    attacks |= sliding_ray(square, occupied,  9, on_file_h); // NE
    attacks |= sliding_ray(square, occupied,  7, on_file_a); // NW
    attacks |= sliding_ray(square, occupied, -7, on_file_h); // SE
    attacks |= sliding_ray(square, occupied, -9, on_file_a); // SW

    attacks
}

pub fn queen_attacks(square: u8, occupied: Bitboard) -> Bitboard {
    rook_attacks(square, occupied) | bishop_attacks(square, occupied)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::bitboard::Square;
    
    #[test]
    fn knight_center() {
        assert_eq!(knight_attacks(Square::E4 as u8).count_ones(), 8);
    }

    #[test]
    fn king_center() {
        assert_eq!(king_attacks(Square::E4 as u8).count_ones(), 8);
    }

    #[test]
    fn white_pawn_center() {
        assert_eq!(white_pawn_attacks(Square::E4 as u8).count_ones(), 2);
    }

    #[test]
    fn black_pawn_center() {
        assert_eq!(black_pawn_attacks(Square::E5 as u8).count_ones(), 2);
    }

    #[test]
    fn rook_blocked() {
        let rook = Square::D4 as u8;
        let blocker = bb(Square::D6 as u8);
        let occupied = blocker;

        let attacks = rook_attacks(rook, occupied);

        assert!(attacks & bb(Square::D5 as u8) != 0);
        assert!(attacks & bb(Square::D6 as u8) != 0);
        assert!(attacks & bb(Square::D7 as u8) == 0);
    }

        #[test]
    fn bishop_center_no_blockers() {
        let attacks = bishop_attacks(Square::D4 as u8, 0);
        assert_eq!(attacks.count_ones(), 13);
    }

        #[test]
    fn bishop_blocked_ne() {
        let bishop = Square::D4 as u8;
        let blocker = bb(Square::F6 as u8);
        let occupied = blocker;

        let attacks = bishop_attacks(bishop, occupied);

        assert!(attacks & bb(Square::E5 as u8) != 0);
        assert!(attacks & bb(Square::F6 as u8) != 0);
        assert!(attacks & bb(Square::G7 as u8) == 0);
    }

        #[test]
    fn bishop_blocked_adjacent() {
        let bishop = Square::D4 as u8;
        let blocker = bb(Square::C5 as u8);
        let occupied = blocker;

        let attacks = bishop_attacks(bishop, occupied);

        assert!(attacks & bb(Square::C5 as u8) != 0);
        assert!(attacks & bb(Square::B6 as u8) == 0);
    }

        #[test]
    fn queen_blocked_mixed() {
        let queen = Square::D4 as u8;
        let occupied =
            bb(Square::D6 as u8) |
            bb(Square::F6 as u8);

        let attacks = queen_attacks(queen, occupied);

        // Rook direction
        assert!(attacks & bb(Square::D5 as u8) != 0);
        assert!(attacks & bb(Square::D6 as u8) != 0);
        assert!(attacks & bb(Square::D7 as u8) == 0);

        // Bishop direction
        assert!(attacks & bb(Square::E5 as u8) != 0);
        assert!(attacks & bb(Square::F6 as u8) != 0);
        assert!(attacks & bb(Square::G7 as u8) == 0);
    }

        #[test]
    fn queen_corner_a1() {
        let attacks = queen_attacks(Square::A1 as u8, 0);

        // Rook
        assert!(attacks & bb(Square::A8 as u8) != 0);
        assert!(attacks & bb(Square::H1 as u8) != 0);

        // Bishop
        assert!(attacks & bb(Square::H8 as u8) != 0);

        // Wraparound checks
        assert!(attacks & bb(Square::H2 as u8) == 0);
    }
}