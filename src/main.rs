fn main() {

    // use chess::chess::Board;
    use chess::board::Board;
    let mut board = Board::new();

    let start = std::time::Instant::now();
    board.load_from_fen("r2q1rk1/pp1b1ppp/2np1n2/2p1p3/2P1P3/2NP1N2/PP1B1PPP/R2Q1RK1 w");
    board.load_from_fen("5n1n/1B5k/2q5/1Pr5/Q2K4/r2N4/N4P1R/8 b");
    let mut count = 0;
    for _ in 0..1000000 {
    // for _ in 0..1 {
        // let mut moves = Vec::new();
        // let _ = board.generate_pesudo_moves(&mut moves);
        let _moves = board.generate_moves();
        // println!("{:#?}", _moves);
        count += 1;
    }
    let end = std::time::Instant::now();
    let duration = end.duration_since(start);
    println!(
        "Time took: {:?} seconds for {} moves",
        duration.as_secs_f64(),
        count
    );
}


