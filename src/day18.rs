#[allow(unused_imports)]
use super::prelude::*;
type Input = FxIndexSet<(usize, usize)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect()
}

const SIZE: usize = 70;

fn try_solve(input: &Input, max: usize) -> Option<usize> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([(0, 0, 0)]);

    while let Some((cost, x, y)) = queue.pop_front() {
        if (x, y) == (SIZE, SIZE) {
            return Some(cost);
        }

        if !seen.insert((x, y)) {
            continue;
        }

        let mut add = |nx, ny| {
            if input.get_index_of(&(nx, ny)).is_none_or(|i| i > max) {
                queue.push_back((cost + 1, nx, ny))
            }
        };

        (x != 0).then(|| add(x - 1, y));
        (x != SIZE).then(|| add(x + 1, y));
        (y != 0).then(|| add(x, y - 1));
        (y != SIZE).then(|| add(x, y + 1));
    }

    None
}

pub fn part1(input: &Input) -> usize {
    try_solve(input, 1024 - 1).unwrap()
}

pub fn part2(input: &Input) -> String {
    let mut min = 0;
    let mut max = input.len();

    while min + 1 < max {
        let mid = (max + min) / 2;
        if try_solve(input, mid).is_some() {
            min = mid;
        } else {
            max = mid;
        }
    }

    let (x, y) = input[max];
    format!("{x},{y}")
}
