#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<u32>>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.split_ascii_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn is_pair_safe(l: u32, r: u32, is_lt: bool) -> bool {
    (l < r) == is_lt && l != r && u32::abs_diff(l, r) <= 3
}

fn is_safe(report: &[u32], is_lt: bool) -> bool {
    report.iter().tuple_windows().all(|(&l, &r)| is_pair_safe(l, r, is_lt))
}

fn is_safe_without(report: &[u32], i: usize, is_lt: bool) -> bool {
    i == 0 || i + 1 == report.len() || is_pair_safe(report[i - 1], report[i + 1], is_lt)
}

fn is_safe2(report: &[u32], is_lt: bool) -> bool {
    let mut cand = 0..report.len();

    for (i, (&l, &r)) in report.iter().tuple_windows().enumerate() {
        if !is_pair_safe(l, r, is_lt) {
            let s = if is_safe_without(report, i, is_lt) { i } else { i + 1 };
            let e = if is_safe_without(report, i + 1, is_lt) { i + 2 } else { i + 1 };
            cand = max(cand.start, s)..min(cand.end, e);
            if cand.is_empty() {
                return false;
            }
        }
    }

    true
}

pub fn part1(input: &Input) -> usize {
    input.iter().filter(|report| is_safe(report, true) || is_safe(report, false)).count()
}

pub fn part2(input: &Input) -> usize {
    input.iter().filter(|report| is_safe2(report, true) || is_safe2(report, false)).count()
}
