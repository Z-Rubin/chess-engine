use chess_engine::board::board::Board;
use chess_engine::movegen::perft::perft;
use chess_engine::movegen::movegen::generate_legal_moves;

fn square_name(sq: u8) -> String {
    let file = (sq % 8) as u8 + b'a';
    let rank = (sq / 8) + 1;
    format!("{}{}", file as char, rank)
}

fn move_to_string(mv: &chess_engine::board::r#move::Move) -> String {
    let mut s = format!("{}{}", square_name(mv.from), square_name(mv.to));
    if let Some(p) = mv.promotion {
        let c = match p {
            1 => 'n',
            2 => 'b',
            3 => 'r',
            4 => 'q',
            _ => '?',
        };
        s.push(c);
    }
    s
}

fn main() {
    // Initialize attack tables
    chess_engine::movegen::init();
    
    // Known correct perft(5) divide from starting position:
    let expected: Vec<(&str, u64)> = vec![
        ("a2a3", 181046),
        ("a2a4", 217832),
        ("b1a3", 198572),
        ("b1c3", 234656),
        ("b2b3", 215255),
        ("b2b4", 216145),
        ("c2c3", 222861),
        ("c2c4", 240082),
        ("d2d3", 328511),
        ("d2d4", 361790),
        ("e2e3", 402988),
        ("e2e4", 405385),
        ("f2f3", 178889),
        ("f2f4", 198473),
        ("g1f3", 233491),
        ("g1h3", 198502),
        ("g2g3", 217210),
        ("g2g4", 214048),
        ("h2h3", 181044),
        ("h2h4", 218829),
    ];
    
    let mut board = Board::startpos();
    let moves = generate_legal_moves(&mut board);
    
    println!("Perft divide comparison at depth 4 from starting position:\n");
    
    let mut total_diff = 0i64;
    
    for (mv_str, exp) in &expected {
        // Find the move
        let mv = moves.iter().find(|m| move_to_string(m) == *mv_str);
        if let Some(mv) = mv {
            let undo = board.make_move(*mv);
            let count = perft(&mut board, 4);
            board.unmake_move(*mv, undo);
            
            let diff = count as i64 - *exp as i64;
            total_diff += diff;
            
            if diff != 0 {
                println!("{}: {} (expected {}), diff = {}", mv_str, count, exp, diff);
            }
        } else {
            println!("{}: MOVE NOT FOUND!", mv_str);
        }
    }
    
    println!("\nTotal difference: {}", total_diff);
    println!("Missing nodes: {}", -total_diff);
}
