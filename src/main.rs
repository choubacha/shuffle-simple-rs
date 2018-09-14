extern crate rand;

use rand::{thread_rng, Rng};
use std::time::Instant;

fn main() {
    let deck = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let mut shuffled_deck = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

    let mut rng = thread_rng();
    let start = Instant::now();
    let mut shuffles = 0u128;

    loop {
        rng.shuffle(&mut shuffled_deck);
        shuffles += 1;
        if shuffled_deck == deck {
            println!("We did it! It only took {} shuffles", shuffles);
            break;
        } else if shuffles % 10_000_000 == 0 {
            let elapsed = start.elapsed();
            println!(
                "{} shuffles later... at ~{} shuffles/sec",
                shuffles,
                1_000 * shuffles
                    / (elapsed.as_secs() * 1_000 + elapsed.subsec_millis() as u64) as u128,
            );
        }
    }
}
