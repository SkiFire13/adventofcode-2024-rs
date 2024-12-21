#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

pub fn part1(input: &Input) -> usize {
    let ((sx, sy), _) = input.iter().find(|&(_, &c)| c == b'S').unwrap();
    let ((ex, ey), _) = input.iter().find(|&(_, &c)| c == b'E').unwrap();

    let mut distances = HashMap::new();
    let mut queue = VecDeque::from([(0, sx, sy)]);

    while let Some((cost, x, y)) = queue.pop_front() {
        if distances.contains_key(&(x, y)) {
            continue;
        }
        distances.insert((x, y), cost);

        if (x, y) == (ex, ey) {
            break;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if input[(nx, ny)] != b'#' {
                queue.push_back((cost + 1, nx as usize, ny as usize));
            }
        }
    }

    let mut count = 0;
    for y in 0..input.h() as isize {
        for x in 0..input.w() as isize {
            if let Some(&c) = distances.get(&(x as usize, y as usize)) {
                for (dx, dy) in
                    [(-2, 0), (-1, 1), (0, 2), (1, 1), (2, 0), (1, -1), (0, -2), (-1, -1)]
                {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx >= 0 && ny >= 0 {
                        if let Some(&c2) = distances.get(&(nx as usize, ny as usize)) {
                            if c2 >= 100 + c + 2 {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

pub fn part2(input: &Input) -> u64 {
    let ((sx, sy), _) = input.iter().find(|&(_, &c)| c == b'S').unwrap();
    let ((ex, ey), _) = input.iter().find(|&(_, &c)| c == b'E').unwrap();

    let mut distances = Grid::with_dimensions(input.w(), input.h());
    let mut queue = VecDeque::from([(1, sx, sy)]);

    while let Some((cost, x, y)) = queue.pop_front() {
        if distances[(x, y)] != 0 {
            continue;
        }
        distances[(x, y)] = cost;

        if (x, y) == (ex, ey) {
            break;
        }

        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if input[(nx, ny)] != b'#' {
                queue.push_back((cost + 1, nx as usize, ny as usize));
            }
        }
    }

    let mut count = 0;
    for y in 0..input.h() as isize {
        for x in 0..input.w() as isize {
            let c = distances[(x, y)];
            if c == 0 {
                continue;
            };

            const N: isize = 20;
            for dx in -N..N + 1 {
                let dy_max = N as isize - dx.abs();
                for dy in -dy_max..dy_max + 1 {
                    let d = dx.abs() + dy.abs();
                    let (nx, ny) = (x + dx, y + dy);
                    if nx < 0 && ny < 0 {
                        continue;
                    }

                    let Some(&c2) = distances.iget((nx, ny)) else { continue };
                    if c2 >= 100 + c + d {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}
