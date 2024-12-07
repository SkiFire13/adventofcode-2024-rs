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

fn is_possible<const PART2: bool>(nums: &[u64], goal: u64) -> bool {
    if nums.len() == 1 {
        return nums[0] == goal;
    }

    let last_idx = nums.len() - 1;
    let last_num = nums[last_idx];

    if last_num > goal {
        return false;
    }

    if PART2 {
        let ten_pow = 10u64.pow(last_num.ilog10() + 1);
        if (goal - last_num) % ten_pow == 0 {
            if is_possible::<PART2>(&nums[..last_idx], goal / ten_pow) {
                return true;
            }
        }
    }

    if goal % last_num == 0 {
        if is_possible::<PART2>(&nums[..last_idx], goal / last_num) {
            return true;
        }
    }

    return last_num < goal && is_possible::<PART2>(&nums[..last_idx], goal - last_num);
}

pub fn part1(input: &Input) -> u64 {
    input.iter().filter(|(n, nums)| is_possible::<false>(nums, *n)).map(|&(n, _)| n).sum()
}

pub fn part2(input: &Input) -> u64 {
    input.iter().filter(|(n, nums)| is_possible::<true>(nums, *n)).map(|&(n, _)| n).sum()
}
