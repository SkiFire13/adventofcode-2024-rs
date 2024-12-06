#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

pub fn part1(input: &Input) -> usize {
    let (x, y) = input.iter_by_row().find(|&c| input[c] == b'^').unwrap();
    let (mut x, mut y) = (x as isize, y as isize);
    let (mut dx, mut dy) = (0, -1);

    let mut pos = HashSet::new();

    loop {
        pos.insert((x, y));
        let (nx, ny) = (x + dx, y + dy);

        if input.iget((nx, ny)).is_none() {
            return pos.len();
        }

        if input[(nx, ny)] == b'#' {
            (dx, dy) = (-dy, dx);
            continue;
        }
        (x, y) = (nx, ny);
    }
}

pub fn part2(input: &Input) -> usize {
    let (sx, sy) = input.iter_by_row().find(|&c| input[c] == b'^').unwrap();

    let (mut x, mut y) = (sx as isize, sy as isize);
    let (mut dx, mut dy) = (0, -1);
    let mut seen = HashSet::new();

    loop {
        let (nx, ny) = (x + dx, y + dy);

        if input.iget((nx, ny)).is_none() {
            break;
        }

        if input[(nx, ny)] == b'#' {
            (dx, dy) = (-dy, dx);
            continue;
        }
        (x, y) = (nx, ny);
        seen.insert((x, y));
    }

    let mut count = 0;

    for (bx, by) in seen {
        let (mut x, mut y) = (sx as isize, sy as isize);
        let (mut dx, mut dy) = (0, -1);

        let mut pos = HashSet::new();
        let mut seen = HashSet::new();

        loop {
            if !seen.insert((x, y, dx, dy)) {
                count += 1;
                break;
            }

            pos.insert((x, y));
            let (nx, ny) = (x + dx, y + dy);

            if input.iget((nx, ny)).is_none() {
                break;
            }

            if input[(nx, ny)] == b'#' || (nx, ny) == (bx, by) {
                (dx, dy) = (-dy, dx);
                continue;
            }
            (x, y) = (nx, ny);
        }
    }

    count
}
