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
        .filter(|update| {
            for (i, &n) in update.iter().enumerate() {
                for &m in update[i + 1..].iter() {
                    if rules.contains(&(m, n)) {
                        return false;
                    }
                }
            }
            true
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    let (rules, updates) = input;

    updates
        .iter()
        .filter_map(|update| {
            let mut invalid = false;
            let mut map = update.iter().map(|&n| (n, HashSet::new())).collect::<HashMap<_, _>>();
            for (i, &n) in update.iter().enumerate() {
                for &m in update[i + 1..].iter() {
                    if rules.contains(&(n, m)) {
                        map.entry(m).or_insert(HashSet::new()).insert(n);
                    } else if rules.contains(&(m, n)) {
                        map.entry(n).or_insert(HashSet::new()).insert(m);
                        invalid = true;
                    }
                }
            }

            if !invalid {
                return None;
            }

            let mut res = Vec::new();
            // Very bad
            while let Some((&n, _)) = map.iter().find(|(_, v)| v.is_empty()) {
                map.remove(&n);
                res.push(n);
                for v in map.values_mut() {
                    v.remove(&n);
                }
            }

            Some(res[res.len() / 2])
        })
        .sum()
}
