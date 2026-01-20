pub type Bitboard = u64;

#[inline(always)]
pub const fn bb(square: u8) -> Bitboard {
    1u64 << square
}

#[inline(always)]
pub const fn rank_of(square: u8) -> u8 {
    square / 8
}

#[inline(always)]
pub const fn file_of(square: u8) -> u8 {
    square % 8
}


#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Square {
    A1 = 0, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

pub const FILE_A: Bitboard = 0x0101010101010101;
pub const FILE_H: Bitboard = 0x8080808080808080;

pub const RANK_1: Bitboard = 0x00000000000000FF;
pub const RANK_2: Bitboard = 0x000000000000FF00;
pub const RANK_3: Bitboard = 0x0000000000FF0000;
pub const RANK_4: Bitboard = 0x00000000FF000000;
pub const RANK_5: Bitboard = 0x000000FF00000000;
pub const RANK_6: Bitboard = 0x0000FF0000000000;
pub const RANK_7: Bitboard = 0x00FF000000000000;
pub const RANK_8: Bitboard = 0xFF00000000000000;

pub const CASTLE_WHITE_KING: u8 = 0b0001;
pub const CASTLE_WHITE_QUEEN: u8 = 0b0010;
pub const CASTLE_BLACK_KING: u8 = 0b0100;
pub const CASTLE_BLACK_QUEEN: u8 = 0b1000;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_mapping() {
        assert_eq!(bb(0), 1u64);
        assert_eq!(bb(63), 1u64 << 63);
        assert_eq!(file_of(0), 0); // A file
        assert_eq!(rank_of(0), 0); // rank 1
        assert_eq!(file_of(63), 7); // H file
        assert_eq!(rank_of(63), 7); // rank 8
    }
}
