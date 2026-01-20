use crate::board::piece::{Color, Piece};

#[derive(Copy, Clone)]
pub struct Undo {
    pub captured: Option<(Color, Piece)>,
    pub side_to_move: Color,
    pub castling_rights: u8,
    pub en_passant_square: Option<u8>,
}