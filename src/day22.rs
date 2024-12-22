#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u64>;

pub fn input_generator(input: &str) -> Input {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn next(mut n: u64) -> u64 {
    n = (n ^ (64 * n)) % 16777216;
    n = (n ^ (n / 32)) % 16777216;
    n = (n ^ (n * 2048)) % 16777216;
    n
}

pub fn part1(input: &Input) -> u64 {
    let mut sum = 0;

    for &(mut n) in input {
        for _ in 0..2000 {
            n = next(n);
        }
        sum += n;
    }

    sum
}

pub fn part2(input: &Input) -> u64 {
    let mut local_counts = FxHashSet::default();
    let mut counts = FxHashMap::default();

    for &(mut n) in input {
        local_counts.clear();

        let b1 = (n % 10) as i8;
        n = next(n);
        let b2 = (n % 10) as i8;
        n = next(n);
        let b3 = (n % 10) as i8;
        n = next(n);
        let mut b4 = (n % 10) as i8;

        let mut d1 = b1 - b2;
        let mut d2 = b2 - b3;
        let mut d3 = b3 - b4;

        for _ in 3..2000 {
            n = next(n);
            let b5 = (n % 10) as i8;

            let d4 = b4 - b5;

            let w = u32::from_ne_bytes([d1, d2, d3, d4].map(|b| b as u8));
            if local_counts.insert(w) {
                *counts.entry(w).or_insert(0) += b5 as u64;
            }

            (d1, d2, d3, b4) = (d2, d3, d4, b5);
        }
    }

    counts.into_values().max().unwrap()
}
