#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion: Option<u8>, // later: piece index
}

pub type MoveList = Vec<Move>;

impl Move {
    pub fn to_uci(&self) -> String {
        let mut result = format!(
            "{}{}{}{}",
            (b'a' + (self.from % 8) as u8) as char,
            (b'1' + (self.from / 8) as u8) as char,
            (b'a' + (self.to % 8) as u8) as char,
            (b'1' + (self.to / 8) as u8) as char,
        );
        
        if let Some(promo) = self.promotion {
            let promo_char = match promo {
                1 => 'n',
                2 => 'b',
                3 => 'r',
                4 => 'q',
                _ => ' ',
            };
            result.push(promo_char);
        }
        
        result
    }

    pub fn from_uci(uci_str: &str) -> Option<Move> {
        if uci_str.len() < 4 {
            return None;
        }
        
        let bytes = uci_str.as_bytes();
        let from_file = (bytes[0] - b'a') as u8;
        let from_rank = (bytes[1] - b'1') as u8;
        let to_file = (bytes[2] - b'a') as u8;
        let to_rank = (bytes[3] - b'1') as u8;
        
        let from = from_rank * 8 + from_file;
        let to = to_rank * 8 + to_file;
        
        let promotion = if uci_str.len() > 4 {
            match bytes[4] {
                b'n' => Some(1), // Knight
                b'b' => Some(2), // Bishop
                b'r' => Some(3), // Rook
                b'q' => Some(4), // Queen
                _ => None,
            }
        } else {
            None
        };
        
        Some(Move { from, to, promotion })
    }
}
