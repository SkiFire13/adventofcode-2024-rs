#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<[usize; 5]>, Vec<[usize; 5]>);

pub fn input_generator(input: &str) -> Input {
    input.as_bytes().chunks(43).partition_map(|c| {
        if c[0] == b'.' {
            let mut h = [0; 5];
            for i in 0..5 {
                let mut b = 36 + i;
                while c[b] == b'#' {
                    h[i] += 1;
                    b -= 6;
                }
            }
            Either::Left(h)
        } else {
            let mut h = [0; 5];
            for i in 0..5 {
                let mut b = i;
                while c[b] == b'#' {
                    h[i] += 1;
                    b += 6;
                }
            }
            Either::Right(h)
        }
    })
}

pub fn part1(input: &Input) -> usize {
    let (keys, locks) = input;
    iproduct!(keys, locks).filter(|&(key, lock)| izip!(key, lock).all(|(k, l)| k + l <= 7)).count()
}
