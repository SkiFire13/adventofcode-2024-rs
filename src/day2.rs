#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<u32>>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn is_safe(report: impl IntoIterator<Item = u32> + Clone) -> bool {
    let mut clone = report.clone().into_iter();
    let ord = clone.next() < clone.next();
    report
        .into_iter()
        .tuple_windows()
        .all(|(l, r)| (l < r) == ord && l != r && u32::abs_diff(l, r) <= 3)
}

pub fn part1(input: &Input) -> usize {
    input.iter().filter(|report| is_safe(report.iter().copied())).count()
}

pub fn part2(input: &Input) -> usize {
    input
        .iter()
        .filter(|report| {
            is_safe(report.iter().copied())
                || (0..report.len()).any(|i| {
                    is_safe(report[..i].iter().copied().chain(report[i + 1..].iter().copied()))
                })
        })
        .count()
}
