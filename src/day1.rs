#[allow(unused_imports)]
use super::prelude::*;
type Input = (Vec<i32>, Vec<i32>);

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(' ').unwrap();
            let l = l.trim().parse::<i32>().unwrap();
            let r = r.trim().parse::<i32>().unwrap();
            (l, r)
        })
        .unzip()
}

pub fn part1(input: &Input) -> i32 {
    let (mut l, mut r) = input.clone();
    l.sort_unstable();
    r.sort_unstable();
    izip!(l, r).map(|(l, r)| (l - r).abs()).sum()
}

pub fn part2(input: &Input) -> i32 {
    let (l, r) = input;
    let counts = r.iter().copied().counts();
    l.iter().map(|&l| l * *counts.get(&l).unwrap_or(&0) as i32).sum()
}
