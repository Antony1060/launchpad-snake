use std::cmp::{max, min};

use rand::RngCore;

pub fn random_range(first: u8, second: u8) -> u8 {
    let mut rand = rand::thread_rng();

    let (min, max) = (min(first, second), max(first, second));

    min + (rand.next_u32() % (max - min) as u32) as u8
}
