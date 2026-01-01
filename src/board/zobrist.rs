use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use once_cell::sync::Lazy;

pub static Z_PIECE: Lazy<[[u64; 64]; 12]> = Lazy::new(|| {
    let mut rng = StdRng::seed_from_u64(0xDEADBEEF);
    let mut table = [[0u64; 64]; 12];

    for p in 0..12 {
        for sq in 0..64 {
            table[p][sq] = rng.random::<u64>();
        }
    }

    table
});

pub static Z_SIDE: Lazy<u64> = Lazy::new(|| {
    let mut rng = StdRng::seed_from_u64(0xDEADBEEF ^ 0xABCDEF);
    rng.random::<u64>()
});

pub static Z_CASTLING: Lazy<[u64; 4]> = Lazy::new(|| {
    let mut rng = StdRng::seed_from_u64(0xDEADBEEF ^ 0x123456);
    [
        rng.random::<u64>(), // White King-side
        rng.random::<u64>(), // White Queen-side
        rng.random::<u64>(), // Black King-side
        rng.random::<u64>(), // Black Queen-side
    ]
});