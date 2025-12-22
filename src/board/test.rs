use rand::seq::IteratorRandom;
use rand::{Rng, rng};
use shakmaty::fen::Fen;
use shakmaty::{Chess, Position};

pub fn random_fen(min_moves: usize, max_moves: usize) -> Fen {
    let mut rng = rng();
    let mut board = Chess::default();

    let moves_to_play = rng.random_range(min_moves..=max_moves);
    for _ in 0..moves_to_play {
        if board.is_game_over() {
            break;
        }

        let mv = board.legal_moves().into_iter().choose(&mut rng).unwrap();

        board = board.play(mv).unwrap();
    }

    Fen::from_position(&board, shakmaty::EnPassantMode::PseudoLegal)
} //

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::board::Board as MyEngine;
    use shakmaty::{Chess, Position};
    use crate::board::rook_magic::init_rook_magics;
    use crate::board::bishop_magic::init_bishop_magics;


    #[test]
    fn move_generation() {

        init_rook_magics();
        init_bishop_magics();

        let mut counter = 0;

        loop {
            let fen = super::random_fen(0, 10);
            let fen_string =
                fen.to_string()
                    .split_whitespace()
                    .take(2)
                    .fold(String::new(), |mut acc, w| {
                        if !acc.is_empty() {
                            acc.push(' ');
                        }
                        acc.push_str(w);
                        acc
                    });

            let shakmaty_board: Chess =
                fen.into_position(shakmaty::CastlingMode::Standard).unwrap();

            let mut board = MyEngine::new();
            board.load_from_fen(&fen_string);

            let mut missed = vec![];

            let moves = shakmaty_board.legal_moves();
            let mut my_moves: HashSet<(u8, u8)> = board
                .generate_moves()
                .iter()
                .map(|mv| (mv.from(), mv.to()))
                .collect();

            for mv in moves {
                match mv {
                    shakmaty::Move::Normal {
                        role,
                        from,
                        capture,
                        to,
                        promotion,
                    } => {
                        // println!("{} {}" , mv.from().unwrap() as usize, mv.to() as usize);
                        let mv = (mv.from().unwrap() as u8, mv.to() as u8);
                        if my_moves.contains(&mv) {
                            my_moves.remove(&mv);
                            continue;
                        } else {
                            missed.push(mv);
                        }
                    }
                    _ => continue,
                }
            }

            if missed.len() > 0 || my_moves.len() > 0 {
                println!("❌ {} {}", counter, fen_string);
                println!("Missed : {:?}", missed);
                println!("Extra : {:?}", my_moves);
                println!("Moves : {:#?}", board.generate_moves().iter().map(|mv| (mv.from() , mv.to())).collect::<Vec<(u8, u8)>>());
                break;
            }
            counter += 1;
        }
    }//
}
