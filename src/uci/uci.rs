use crate::board::board::Board;
use crate::board::r#move::Move;
use crate::search::search::search;
use std::io::{self, Write};

pub fn uci_loop() {
    let mut board = Board::startpos();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = input.trim();

        match cmd {
            "uci" => {
                println!("id name MyChessEngine");
                println!("id author Zac Rubin");
                println!("uciok");
            }
            "isready" => {
                println!("readyok");
            }
            "ucinewgame" => {
                board = Board::startpos();
            }

            _ if cmd.starts_with("position startpos") => {
                board = Board::startpos();
                
                // Handle moves if present
                if let Some(moves_idx) = cmd.find("moves") {
                    let moves_str = &cmd[moves_idx + 5..].trim();
                    let move_list: Vec<&str> = moves_str.split_whitespace().collect();
                    
                    for move_uci in move_list {
                        if let Some(mv) = Move::from_uci(move_uci) {
                            let _ = board.make_move(mv);
                        }
                    }
                }
            }

            _ if cmd.starts_with("go depth") => {
                let depth: u32 = cmd.split_whitespace().last().unwrap().parse().unwrap();
                let (_score, best) = search(&mut board, depth);

                if let Some(mv) = best {
                    println!("bestmove {}", mv.to_uci());
                }
            }

            "quit" => break,

            _ => {}
        }

        io::stdout().flush().unwrap();
    }
}