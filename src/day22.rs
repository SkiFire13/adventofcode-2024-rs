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
    let mut counts =
        (0..19 * 19 * 19 * 19).map(|_| std::sync::atomic::AtomicU32::new(0)).collect::<Vec<_>>();
    let cores = std::thread::available_parallelism().unwrap().get();
    input.par_chunks((input.len() + cores - 1) / cores).with_max_len(1).for_each(|chunk| {
        for &(mut n) in chunk {
            let mut seen = [0u64; (19 * 19 * 19 * 19 + 63) / 64];

            let b1 = n % 10;
            n = next(n);
            let b2 = n % 10;
            n = next(n);
            let b3 = n % 10;
            n = next(n);
            let mut b4 = n % 10;

            let mut d1 = 9 + b1 - b2;
            let mut d2 = 9 + b2 - b3;
            let mut d3 = 9 + b3 - b4;

            for _ in 3..2000 {
                n = next(n);
                let b5 = n % 10;

                let d4 = 9 + b4 - b5;

                let idx = (d1 + 19 * (d2 + 19 * (d3 + 19 * d4))) as usize;
                if seen[idx / 64] & (1 << (idx % 64)) == 0 {
                    seen[idx / 64] |= 1 << (idx % 64);
                    counts[idx].fetch_add(b5 as u32, std::sync::atomic::Ordering::Relaxed);
                }

                (d1, d2, d3, b4) = (d2, d3, d4, b5);
            }
        }
    });

    counts.iter_mut().map(|a| *a.get_mut()).max().unwrap() as u64
}
