#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

pub fn part1(input: &Input) -> usize {
    let mut seen = HashSet::new();
    let mut tot = 0;

    for ((x, y), &c) in input.iter() {
        if seen.insert((x, y)) {
            let area_pre = seen.len() - 1;
            let mut perimeter = 0;
            let mut stack = vec![(x, y)];

            while let Some((x, y)) = stack.pop() {
                let mut nc = 0;
                for (x, y) in input.plus_neighbours((x, y)) {
                    if input[(x, y)] == c {
                        nc += 1;
                        if seen.insert((x, y)) {
                            stack.push((x, y));
                        }
                    }
                }
                perimeter += 4 - nc;
            }

            let area = seen.len() - area_pre;
            tot += area * perimeter;
        }
    }

    tot
}

pub fn part2(input: &Input) -> usize {
    let mut seen = HashSet::new();
    let mut tot = 0;

    for ((x, y), &c) in input.iter() {
        let (x, y) = (x as isize, y as isize);
        if seen.insert((x, y)) {
            let area_pre = seen.len() - 1;
            let mut stack = vec![(x, y)];
            let mut perimeter_l = Vec::new();
            let mut perimeter_r = Vec::new();
            let mut perimeter_t = Vec::new();
            let mut perimeter_b = Vec::new();

            while let Some((x, y)) = stack.pop() {
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let (x, y) = (x + dx, y + dy);
                    if let Some(&c2) = input.iget((x, y)) {
                        if c == c2 {
                            if seen.insert((x, y)) {
                                stack.push((x, y));
                            }
                            continue;
                        }
                    }
                    if dx == -1 {
                        perimeter_l.push((x, y));
                    } else if dx == 1 {
                        perimeter_r.push((x, y));
                    } else if dy == -1 {
                        perimeter_t.push((x, y));
                    } else if dy == 1 {
                        perimeter_b.push((x, y));
                    }
                }
            }

            perimeter_l.sort_by_key(|&(x, y)| (x, y));
            perimeter_r.sort_by_key(|&(x, y)| (x, y));
            perimeter_t.sort_by_key(|&(x, y)| (y, x));
            perimeter_b.sort_by_key(|&(x, y)| (y, x));

            let perimeter =
                perimeter_l.chunk_by(|&(x1, y1), &(x2, y2)| x1 == x2 && y1 + 1 >= y2).count()
                    + perimeter_r.chunk_by(|&(x1, y1), &(x2, y2)| x1 == x2 && y1 + 1 >= y2).count()
                    + perimeter_t.chunk_by(|&(x1, y1), &(x2, y2)| y1 == y2 && x1 + 1 >= x2).count()
                    + perimeter_b.chunk_by(|&(x1, y1), &(x2, y2)| y1 == y2 && x1 + 1 >= x2).count();
            let area = seen.len() - area_pre;

            tot += area * perimeter;
        }
    }

    tot
}
