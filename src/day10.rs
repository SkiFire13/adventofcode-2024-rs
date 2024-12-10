#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

pub fn part1(input: &Input) -> usize {
    let mut tot = 0;

    for (p, &c) in input.iter() {
        if c == b'0' {
            let mut reachable = HashSet::from([p]);

            for c in b'1'..b'9' + 1 {
                for p in std::mem::take(&mut reachable) {
                    for p in input.plus_neighbours(p) {
                        if input[p] == c {
                            reachable.insert(p);
                        }
                    }
                }
            }

            tot += reachable.len();
        }
    }

    tot
}

pub fn part2(input: &Input) -> usize {
    let mut tot = 0;

    for (p, &c) in input.iter() {
        if c == b'0' {
            let mut reachable = HashMap::from([(p, 1)]);

            for c in b'1'..b'9' + 1 {
                for (p, n) in std::mem::take(&mut reachable) {
                    for p in input.plus_neighbours(p) {
                        if input[p] == c {
                            *reachable.entry(p).or_insert(0) += n;
                        }
                    }
                }
            }

            tot += reachable.values().copied().sum::<usize>();
        }
    }

    tot
}
