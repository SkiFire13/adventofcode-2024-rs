#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

pub fn part1(input: &Input) -> usize {
    let mut count = 0;

    for y in 0..input.h() as isize {
        for x in 0..input.w() as isize {
            if input[(x, y)] == b'X' {
                let dirs = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
                for (dx, dy) in dirs {
                    if input.iget((x + dx, y + dy)) == Some(&b'M')
                        && input.iget((x + 2 * dx, y + 2 * dy)) == Some(&b'A')
                        && input.iget((x + 3 * dx, y + 3 * dy)) == Some(&b'S')
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

pub fn part2(input: &Input) -> usize {
    let mut count = 0;

    for y in 1..input.h() - 1 {
        for x in 1..input.w() - 1 {
            if input[(x, y)] == b'A' {
                let tl = input[(x - 1, y - 1)];
                let bl = input[(x - 1, y + 1)];
                let br = input[(x + 1, y + 1)];
                let tr = input[(x + 1, y - 1)];
                if matches!(&[tl, bl, br, tr], b"MMSS" | b"SMMS" | b"SSMM" | b"MSSM") {
                    count += 1;
                }
            }
        }
    }

    count
}
