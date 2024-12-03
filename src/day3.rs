#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Op>;

pub enum Op {
    Do,
    Dont,
    Mul(u32, u32),
}

pub fn input_generator(input: &str) -> Input {
    let re = Regex::new(r#"do\(\)|don't\(\)|mul\((\d+),(\d+)\)"#).unwrap();
    re.captures_iter(input)
        .map(|cap| match &cap[0] {
            "do()" => Op::Do,
            "don't()" => Op::Dont,
            _ => Op::Mul(cap[1].parse().unwrap(), cap[2].parse().unwrap()),
        })
        .collect()
}

pub fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|op| match op {
            Op::Mul(l, r) => l * r,
            _ => 0,
        })
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    let mut sum = 0;
    let mut enabled = true;

    for op in input {
        match op {
            Op::Do => enabled = true,
            Op::Dont => enabled = false,
            Op::Mul(l, r) if enabled => sum += l * r,
            _ => {}
        }
    }

    sum
}
