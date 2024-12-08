#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

pub fn part1(input: &Input) -> usize {
    let mut antinodes_seen = HashSet::new();

    for ((x, y), &c) in input.iter() {
        let (x, y) = (x as isize, y as isize);
        if c != b'.' {
            for ((x2, y2), &c2) in input.iter() {
                let (x2, y2) = (x2 as isize, y2 as isize);
                if c2 == c && (x, y) != (x2, y2) {
                    let (ax, ay) = (x2 + (x2 - x), y2 + (y2 - y));
                    if 0 <= ax && ax < input.w() as isize && 0 <= ay && ay < input.h() as isize {
                        antinodes_seen.insert((ax, ay));
                    }
                    let (ax, ay) = (x - (x2 - x), y - (y2 - y));
                    if 0 <= ax && ax < input.w() as isize && 0 <= ay && ay < input.h() as isize {
                        antinodes_seen.insert((ax, ay));
                    }
                }
            }
        }
    }

    antinodes_seen.len()
}

pub fn part2(input: &Input) -> usize {
    let mut antinodes_seen = HashSet::new();

    for ((x, y), &c) in input.iter() {
        let (x, y) = (x as isize, y as isize);
        if c != b'.' {
            for ((x2, y2), &c2) in input.iter() {
                let (x2, y2) = (x2 as isize, y2 as isize);
                if c2 == c && (x, y) != (x2, y2) {
                    let (dx, dy) = (x2 - x, y2 - y);
                    let gcd = num::integer::gcd(dx, dy);
                    let (dx, dy) = (dx / gcd, dy / gcd);

                    let (mut ax, mut ay) = (x + dx, y + dy);
                    while 0 <= ax && ax < input.w() as isize && 0 <= ay && ay < input.h() as isize {
                        antinodes_seen.insert((ax, ay));
                        (ax, ay) = (ax + dx, ay + dy);
                    }
                    let (mut ax, mut ay) = (x - dx, y - dy);
                    while 0 <= ax && ax < input.w() as isize && 0 <= ay && ay < input.h() as isize {
                        antinodes_seen.insert((ax, ay));
                        (ax, ay) = (ax - dx, ay - dy);
                    }
                }
            }
        }
    }

    antinodes_seen.len()
}
