use chess::board::Move;
use chess::board::bishop_magic::init_bishop_magics;
use chess::board::rook_magic::init_rook_magics;
use std::io::{self, Write};

fn square_to_index(square: &str) -> Option<u8> {
    let bytes = square.as_bytes();
    if bytes.len() != 2 {
        return None;
    }

    let file = bytes[0];
    let rank = bytes[1];

    if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
        return None;
    }

    let file_idx = file - b'a';        // a = 0, h = 7
    let rank_idx = rank - b'1';        // 1 = 0, 8 = 7

    Some(rank_idx * 8 + file_idx)
}

fn main() {
    init_bishop_magics();
    init_rook_magics();

    let mut board = chess::board::Board::new();

    loop {
        io::stdout().flush().unwrap();

        // Read Input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read prompt");

        let input = input.trim();

        if input == "uci" {
            // Initialization
            println!("id name QueenFish 2.0");
            println!("id Author Omar Emad (om3x4)");
            println!("uciok");
        } else if input == "isready" {
            println!("readyok");
        } else if input == "ucinewgame" {
            // println!("ok");
        } else if input.starts_with("position") {
            let tokens: Vec<&str> = input.split_whitespace().collect();

            board.reset_to_default();

            let mut idx = 1;

            if tokens[idx] == "startpos" {
                idx += 1;
            } else if tokens[idx] == "fen" {
                let fen = tokens[idx + 1..idx + 7].join(" ");
                board.load_from_fen(&fen);
                idx += 7;
            }

            if idx < tokens.len() && tokens[idx] == "moves" {
                idx += 1;
                while idx < tokens.len() {
                    let mv = Move::from_uci(tokens[idx] , &board);
                    board.make_move(mv);
                    idx += 1;
                }
            }
        } else if input.starts_with("go") {
            let mut depth = 64;
            let mut nodes = 35_000_000;
            let args = input.split(' ').collect::<Vec<&str>>();

            let depth_parsed = args
                .iter()
                .position(|&x| x == "depth")
                .and_then(|i| args.get(i + 1));

            let nodes_parsed = args
                .iter()
                .position(|&x| x == "nodes")
                .and_then(|i| args.get(i + 1));

            if let Some(depth_str) = depth_parsed {
                depth = depth_str.parse::<i32>().unwrap();
            }

            if let Some(nodes_str) = nodes_parsed {
                nodes = nodes_str.parse::<u64>().unwrap();
            }

            println!(
                "bestmove {}",
                board
                    .engine(depth, true, false, true, true, false, nodes)
                    .to_uci()
            );
        } else if input == "quit" {
            break;
        }
    }
}
