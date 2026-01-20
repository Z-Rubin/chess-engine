use crate::board::board::Board;
use crate::movegen::movegen::generate_legal_moves;

/// Classic perft recursion
pub fn perft(board: &mut Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = generate_legal_moves(board);

    for mv in moves {
        let undo = board.make_move(mv);
        nodes += perft(board, depth - 1);
        board.unmake_move(mv, undo);
    }

    nodes
}

/// Perft with move breakdown at root
pub fn perft_divide(board: &mut Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    let moves = generate_legal_moves(board);

    for mv in &moves {
        let undo = board.make_move(*mv);
        let count = if depth > 1 {
            perft(board, depth - 1)
        } else {
            1
        };
        board.unmake_move(*mv, undo);
        
        println!("{}{}: {}", square_name(mv.from), square_name(mv.to), count);
        nodes += count;
    }
    
    println!("\nTotal: {}", nodes);
    nodes
}

fn square_name(sq: u8) -> String {
    let file = (sq % 8) as u8 + b'a';
    let rank = (sq / 8) + 1;
    format!("{}{}", file as char, rank)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::board::Board;
    use crate::movegen::movegen::generate_legal_moves;
    use std::collections::HashSet;

    #[test]
    fn no_duplicate_moves() {
        let mut board = Board::startpos();
        let moves = generate_legal_moves(&mut board);
        
        let mut seen = HashSet::new();
        for mv in &moves {
            let key = (mv.from, mv.to);
            assert!(!seen.contains(&key), "Duplicate move: {:?}", mv);
            seen.insert(key);
        }
        
        assert_eq!(moves.len(), 20, "Should have 20 moves from starting position");
    }

    #[test]
    fn perft_startpos() {
        let mut board = Board::startpos();

        // Known perft counts
        assert_eq!(perft(&mut board, 1), 20);
        assert_eq!(perft(&mut board, 2), 400);
        assert_eq!(perft(&mut board, 3), 8_902);
        assert_eq!(perft(&mut board, 4), 197_281);
        assert_eq!(perft(&mut board, 5), 4_865_609);
        assert_eq!(perft(&mut board, 6), 119_060_324);
    }
}
