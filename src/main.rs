extern crate rand;

use rand::prelude::*;
use rand::prng::XorShiftRng;
use std::time::Instant;

#[inline(always)]
fn shuffle(deck: &mut [i32], rng: &mut impl Rng) {
    for i in 0..(deck.len() - 2) {
        let j = rng.gen_range(i as u8, deck.len() as u8) as usize;
        let swap = deck[i];
        deck[i] = deck[j];
        deck[j] = swap;
    }
}

#[inline(always)]
fn compare(deck: &[i32], other: &[i32]) -> bool {
    if deck.len() != other.len() {
        return false;
    }

    for i in 0..deck.len() {
        if deck[i] != other[i] {
            return false;
        }
    }
    true
}

fn main() {
    let deck = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let mut shuffled_deck = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

    let mut rng = XorShiftRng::from_entropy();
    let start = Instant::now();
    let mut shuffles = 0u64;

    loop {
        shuffle(&mut shuffled_deck, &mut rng);
        shuffles += 1;
        if compare(&deck, &shuffled_deck) {
            println!("We did it! It only took {} shuffles", shuffles);
            break;
        } else if shuffles % 10_000_000 == 0 {
            let elapsed = start.elapsed();
            println!(
                "{} shuffles later... at ~{} shuffles/sec",
                shuffles,
                1_000 * shuffles / (elapsed.as_secs() * 1_000 + elapsed.subsec_millis() as u64),
            );
        }
    }
}
