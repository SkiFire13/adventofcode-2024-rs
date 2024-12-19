#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = (Vec<&'a str>, Vec<&'a str>);

pub fn input_generator(input: &str) -> Input {
    let (pats, designs) = input.split_once("\n\n").unwrap();
    let pats = pats.split(", ").collect();
    let designs = designs.lines().collect();
    (pats, designs)
}

pub fn part1(input: &Input) -> usize {
    let (pats, designs) = input;
    let regex = regex::Regex::new(&format!("^({})+$", pats.iter().join("|"))).unwrap();
    designs.iter().filter(|goal| regex.is_match(goal)).count()
}

pub fn part2(input: &Input) -> usize {
    fn count<'a>(goal: &'a str, pats: &[&str], cache: &mut FxHashMap<&'a str, usize>) -> usize {
        if let Some(&res) = cache.get(goal) {
            return res;
        }

        let res = pats
            .iter()
            .filter_map(|pat| goal.strip_prefix(pat))
            .map(|rest| count(rest, pats, cache))
            .sum();

        cache.insert(goal, res);

        res
    }

    let (avail, goals) = input;
    let mut cache = FxHashMap::from_iter([("", 1)]);
    goals.iter().map(|goal| count(goal, &avail, &mut cache)).sum()
}
