#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u64>;

pub fn input_generator(input: &str) -> Input {
    input.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn solve(input: &Input, steps: usize) -> u64 {
    let mut nums = HashMap::new();
    for &n in input {
        *nums.entry(n).or_insert(0) += 1;
    }
    for _ in 0..steps {
        for (n, c) in std::mem::take(&mut nums) {
            match n {
                0 => *nums.entry(1).or_insert(0) += c,
                _ if n.ilog10() % 2 == 1 => {
                    let pow10 = 10u64.pow((n.ilog10() + 1) / 2);
                    *nums.entry(n / pow10).or_insert(0) += c;
                    *nums.entry(n % pow10).or_insert(0) += c;
                }
                _ => *nums.entry(n * 2024).or_insert(0) += c,
            }
        }
    }
    nums.into_values().sum()
}

pub fn part1(input: &Input) -> u64 {
    solve(input, 25)
}

pub fn part2(input: &Input) -> u64 {
    solve(input, 75)
}
