use chess_engine::board::board::Board;
use chess_engine::movegen::perft::perft;

fn main() {
    let mut board = Board::startpos();
    
    println!("Checking perft depths:");
    println!("Depth 1: {} (expected 20)", perft(&mut board, 1));
    println!("Depth 2: {} (expected 400)", perft(&mut board, 2));
    println!("Depth 3: {} (expected 8902)", perft(&mut board, 3));
    println!("Depth 4: {} (expected 197281)", perft(&mut board, 4));
    println!("Depth 5: {} (expected 4865609)", perft(&mut board, 5));
    println!("Depth 6: {} (expected 119060324)", perft(&mut board, 6));
    println!("Depth 7: {} (expected 3195901860)", perft(&mut board, 7));
    println!("Depth 8: {} (expected 84998978956)", perft(&mut board, 8));
}
