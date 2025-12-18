fn main(){
    use smallvec::SmallVec;
    use chess::board::*;
    let mut board = Board::new();
    board.load_from_fen("2b2r2/rp1nb1p1/1q1p1n1k/p1p1Np2/1PQPp3/P1N1P3/2P2PPP/2RK1B1R w");

    let start = std::time::Instant::now();
    for _ in 0..10_000_000 {
        let mut moves = SmallVec::new();
        let _ = board.generate_pesudo_moves(&mut moves);
    }
    dbg!(start.elapsed());


}