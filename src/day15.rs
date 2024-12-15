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
                b'#' => break,
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
    for &op in *ops {
        let (dx, dy) = match op {
            b'^' => (0, -1),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            b'>' => (1, 0),
            b'\n' => continue,
            _ => unreachable!(),
        };

        fn move2(grid: &mut Grid<u8>, x: isize, y: isize, dx: isize, dy: isize) -> bool {
            match grid[(x, y)] {
                b'#' => false,
                b'.' => true,
                b'[' => {
                    if dx == 0 {
                        if move2(grid, x, y + dy, dx, dy) && move2(grid, x + 1, y + dy, dx, dy) {
                            grid[(x, y)] = b'.';
                            grid[(x + 1, y)] = b'.';
                            grid[(x, y + dy)] = b'[';
                            grid[(x + 1, y + dy)] = b']';
                            true
                        } else {
                            false
                        }
                    } else {
                        if move2(grid, x + 2, y, dx, dy) {
                            grid[(x, y)] = b'.';
                            grid[(x + 1, y)] = b'[';
                            grid[(x + 2, y)] = b']';
                            true
                        } else {
                            false
                        }
                    }
                }
                b']' => {
                    if dx == 0 {
                        if move2(grid, x, y + dy, dx, dy) && move2(grid, x - 1, y + dy, dx, dy) {
                            grid[(x, y)] = b'.';
                            grid[(x - 1, y)] = b'.';
                            grid[(x, y + dy)] = b']';
                            grid[(x - 1, y + dy)] = b'[';
                            true
                        } else {
                            false
                        }
                    } else {
                        if move2(grid, x - 2, y, dx, dy) {
                            grid[(x - 2, y)] = b'[';
                            grid[(x - 1, y)] = b']';
                            grid[(x, y)] = b'.';
                            true
                        } else {
                            false
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        let mut new_grid = grid.clone();
        if move2(&mut new_grid, x + dx, y + dy, dx, dy) {
            for ((x, y), &c) in new_grid.iter() {
                if c == b'[' {
                    assert_eq!(new_grid[(x + 1, y)], b']');
                }
            }
            grid = new_grid;
            (x, y) = (x + dx, y + dy);
        }

        // fn check_move(grid: &Grid<u8>, x: isize, y: isize, dx: isize, dy: isize, r: bool) -> bool {
        //     match grid[(x, y)] {
        //         b'#' => false,
        //         b'.' => true,
        //         b'[' => {
        //             if dx == 0 {
        //                 if r && !check_move(grid, x + 1, y, dx, dy, false) {
        //                     return false;
        //                 }
        //                 check_move(grid, x + dx, y + dy, dx, dy, true)
        //             } else {
        //                 check_move(grid, x + 2 * dx, y + 2 * dy, dx, dy, false)
        //             }
        //         }
        //         b']' => {
        //             if dx == 0 {
        //                 if r && !check_move(grid, x - 1, y, dx, dy, false) {
        //                     return false;
        //                 }
        //                 check_move(grid, x + dx, y + dy, dx, dy, true)
        //             } else {
        //                 check_move(grid, x + 2 * dx, y + 2 * dy, dx, dy, false)
        //             }
        //         }
        //         _ => unreachable!(),
        //     }
        // }
        // fn do_move(grid: &mut Grid<u8>, x: isize, y: isize, dx: isize, dy: isize, c: u8, r: bool) {
        //     match grid[(x, y)] {
        //         b'#' => unreachable!(),
        //         b'.' => grid[(x, y)] = c,
        //         b'[' => {
        //             if dx == 0 {
        //                 if r {
        //                     do_move(grid, x + 1, y, dx, dy, b'.', false);
        //                 }
        //                 grid[(x, y)] = c;
        //                 do_move(grid, x + dx, y + dy, dx, dy, b'[', true);
        //             } else {
        //                 grid[(x + dx, y + dy)] = b'[';
        //                 do_move(grid, x + 2 * dx, y + 2 * dy, dx, dy, b']', false);
        //             }
        //         }
        //         b']' => {
        //             if dx == 0 {
        //                 if r {
        //                     do_move(grid, x - 1, y, dx, dy, b'.', false);
        //                 }
        //                 grid[(x, y)] = c;
        //                 do_move(grid, x + dx, y + dy, dx, dy, b'[', true);
        //             } else {
        //                 grid[(x + dx, y + dy)] = b']';
        //                 do_move(grid, x + 2 * dx, y + 2 * dy, dx, dy, b'[', false);
        //             }
        //         }
        //         _ => unreachable!(),
        //     }
        // }

        // if check_move(&grid, x + dx, y + dy, dx, dy, true) {
        //     do_move(&mut grid, x + dx, y + dy, dx, dy, b'.', true);
        //     (x, y) = (x + dx, y + dy);
        // }
    }

    grid.iter().filter(|(_, &c)| c == b'[').map(|((x, y), _)| 100 * y + x).sum()
}
