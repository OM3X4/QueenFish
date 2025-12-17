fn main() {
    use bishop_magic::init_bishop_magics;
    use chess::board::*;
    use rook_magic::init_rook_magics;

    init_bishop_magics();
    init_rook_magics();

    let start = std::time::Instant::now();

    let mut board = Board::new();
    board.load_from_fen("5b2/3k1b2/7p/1n1PR3/2p3q1/5Q1R/2pr1N1K/7N w");
    // let best_move = board.engine_multithreaded();
    let best_move = board.engine_multithreaded();
    // for _ in 0..20_000_000 {
    //     // let mut moves: Vec<Move> = Vec::with_capacity(256);
    //     let mut moves: SmallVec<[Move; 256]> = SmallVec::new();
    //     board.generate_pesudo_moves(&mut moves);
    // }
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    println!("Time took: {:?} seconds", duration.as_secs_f64(),);
    println!("{:?}", best_move.to_uci());
}
