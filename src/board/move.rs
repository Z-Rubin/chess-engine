#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion: Option<u8>, // later: piece index
}

pub type MoveList = Vec<Move>;