#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(u64, Vec<u64>)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (n, rest) = line.split_once(": ").unwrap();
            let n = n.parse().unwrap();
            let rest = rest.split(' ').map(|m| m.parse().unwrap()).collect();
            (n, rest)
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    fn is_possible(acc: u64, rem: &[u64], goal: u64) -> bool {
        if acc > goal {
            return false;
        }

        if rem.len() == 0 {
            return acc == goal;
        }

        is_possible(acc + rem[0], &rem[1..], goal) || is_possible(acc * rem[0], &rem[1..], goal)
    }
    input.iter().filter(|(n, nums)| is_possible(nums[0], &nums[1..], *n)).map(|&(n, _)| n).sum()
}

pub fn part2(input: &Input) -> u64 {
    fn concat(n: u64, m: u64) -> u64 {
        m + n * 10u64.pow(m.ilog10() + 1)
    }

    fn is_possible(acc: u64, rem: &[u64], goal: u64) -> bool {
        if acc > goal {
            return false;
        }

        if rem.len() == 0 {
            return acc == goal;
        }

        is_possible(acc + rem[0], &rem[1..], goal)
            || is_possible(acc * rem[0], &rem[1..], goal)
            || is_possible(concat(acc, rem[0]), &rem[1..], goal)
    }

    input.iter().filter(|(n, nums)| is_possible(nums[0], &nums[1..], *n)).map(|&(n, _)| n).sum()
}
