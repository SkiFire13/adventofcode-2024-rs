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
                for (dx, dy) in
                    [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]
                {
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
                let cm = (input[(x - 1, y - 1)] == b'M') as usize
                    + (input[(x - 1, y + 1)] == b'M') as usize
                    + (input[(x + 1, y - 1)] == b'M') as usize
                    + (input[(x + 1, y + 1)] == b'M') as usize;

                let cs = (input[(x - 1, y - 1)] == b'S') as usize
                    + (input[(x - 1, y + 1)] == b'S') as usize
                    + (input[(x + 1, y - 1)] == b'S') as usize
                    + (input[(x + 1, y + 1)] == b'S') as usize;

                if cm == 2 && cs == 2 && input[(x - 1, y - 1)] != input[(x + 1, y + 1)] {
                    count += 1;
                }
            }
        }
    }

    count
}
