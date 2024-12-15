#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = (Grid<u8>, (usize, usize), &'a [u8]);

pub fn input_generator(input: &str) -> Input {
    let (grid, ops) = input.split_once("\n\n").unwrap();
    let mut grid = Grid::from_input_chars(grid, |c, _, _| c as u8);
    let ((x, y), _) = grid.iter().find(|(_, &c)| c == b'@').unwrap();
    grid[(x, y)] = b'.';
    (grid, (x, y), ops.trim().as_bytes())
}

pub fn part1(input: &Input) -> usize {
    let (grid, (x, y), ops) = input;
    let mut grid = grid.clone();

    let (mut x, mut y) = (*x as isize, *y as isize);
    for &op in *ops {
        let (dx, dy) = match op {
            b'^' => (0, -1),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'\n' => continue,
            _ => unreachable!(),
        };
        let (mut fx, mut fy) = (x + dx, y + dy);
        loop {
            match grid[(fx, fy)] {
                b'O' => (fx, fy) = (fx + dx, fy + dy),
                b'.' => {
                    (x, y) = (x + dx, y + dy);
                    grid[(fx, fy)] = b'O';
                    grid[(x, y)] = b'.';
                    break;
                }
                b'#' => {
                    break;
                }
                _ => unreachable!(),
            }
        }
    }

    grid.iter().filter(|(_, &c)| c == b'O').map(|((x, y), _)| 100 * y + x).sum()
}

pub fn part2(input: &Input) -> usize {
    let (init_grid, (x, y), ops) = input;

    let mut grid = Grid::with_dimensions(init_grid.w() * 2, init_grid.h());
    for y in 0..init_grid.h() {
        for x in 0..init_grid.w() {
            match init_grid[(x, y)] {
                b'O' => {
                    grid[(2 * x, y)] = b'[';
                    grid[(2 * x + 1, y)] = b']';
                }
                b'.' => {
                    grid[(2 * x, y)] = b'.';
                    grid[(2 * x + 1, y)] = b'.';
                }
                b'#' => {
                    grid[(2 * x, y)] = b'#';
                    grid[(2 * x + 1, y)] = b'#';
                }
                _ => unreachable!(),
            }
        }
    }

    let (mut x, mut y) = (2 * *x as isize, *y as isize);
    'outer: for &op in *ops {
        let (dx, dy) = match op {
            b'^' => (0, -1),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'\n' => continue,
            _ => unreachable!(),
        };

        let (nx, ny) = (x + dx, y + dy);

        if grid[(nx, ny)] != b'.' {
            let mut to_move = FxIndexSet::from_iter([(nx, ny)]);
            let mut index = 0;

            while let Some(&(x, y)) = to_move.get_index(index) {
                index += 1;
                match grid[(x, y)] {
                    b'#' => continue 'outer,
                    b'.' => {}
                    b'[' => {
                        to_move.insert((x + 1, y));
                        if grid[(x + dx, y + dy)] != b'.' {
                            to_move.insert((x + dx, y + dy));
                        }
                    }
                    b']' => {
                        to_move.insert((x - 1, y));
                        if grid[(x + dx, y + dy)] != b'.' {
                            to_move.insert((x + dx, y + dy));
                        }
                    }
                    _ => unreachable!(),
                }
            }

            for &(x, y) in to_move.iter().rev() {
                grid[(x + dx, y + dy)] = grid[(x, y)];
                grid[(x, y)] = b'.';
            }
        }

        (x, y) = (nx, ny);
    }

    grid.iter().filter(|(_, &c)| c == b'[').map(|((x, y), _)| 100 * y + x).sum()
}
