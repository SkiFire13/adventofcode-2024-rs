#[allow(unused_imports)]
use super::prelude::*;
type Input = (HashSet<(u32, u32)>, Vec<Vec<u32>>);

pub fn input_generator(input: &str) -> Input {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|rule| {
            let (before, after) = rule.split_once('|').unwrap();
            (before.parse().unwrap(), after.parse().unwrap())
        })
        .collect();
    let updates = updates
        .lines()
        .map(|update| update.split(',').map(|elem| elem.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

pub fn part1(input: &Input) -> u32 {
    let (rules, updates) = input;

    updates
        .iter()
        .filter(|update| update.is_sorted_by(|&l, &r| rules.contains(&(l, r))))
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    let (rules, updates) = input;

    updates
        .iter()
        .filter(|update| !update.is_sorted_by(|&l, &r| rules.contains(&(l, r))))
        .map(|update| {
            let mut update = update.clone();
            let len = update.len();
            let (_, &mut mid, _) =
                update.select_nth_unstable_by(len / 2, |&l, &r| match rules.contains(&(l, r)) {
                    true => Ordering::Less,
                    false => Ordering::Greater,
                });
            mid
        })
        .sum()
}
