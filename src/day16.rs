#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

pub fn part1(input: &Input) -> usize {
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::from([(Reverse(0), (1isize, input.h() as isize - 2), (1, 0))]);

    while let Some((Reverse(c), (x, y), (dx, dy))) = queue.pop() {
        if (x, y) == (input.w() as isize - 2, 1) {
            return c;
        }

        if !seen.insert((x, y, dx, dy)) {
            continue;
        }

        queue.push((Reverse(c + 1000), (x, y), (dy, -dx)));
        queue.push((Reverse(c + 1000), (x, y), (-dy, dx)));

        let (mut nx, mut ny, mut nc) = (x + dx, y + dy, c + 1);
        while input[(nx, ny)] != b'#' {
            queue.push((Reverse(nc), (nx, ny), (dx, dy)));
            (nx, ny, nc) = (nx + dx, ny + dy, nc + 1);
        }
    }

    unreachable!();
}

pub fn part2(input: &Input) -> usize {
    let mut prev = HashMap::new();
    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::from([(Reverse(0), (1isize, input.h() as isize - 2), (1, 0))]);

    let mut best_c = None;

    while let Some((Reverse(c), (x, y), (dx, dy))) = queue.pop() {
        if (x, y) == (input.w() as isize - 2, 1) {
            best_c = Some(c);
        }

        if best_c.is_some_and(|best_c| best_c < c) {
            break;
        }

        if !seen.insert((x, y, dx, dy)) {
            continue;
        }

        queue.push((Reverse(c + 1000), (x, y), (dy, -dx)));
        prev.entry((x, y, dy, -dx, c + 1000)).or_insert_with(Vec::new).push((x, y, dx, dy, c));
        queue.push((Reverse(c + 1000), (x, y), (-dy, dx)));
        prev.entry((x, y, -dy, dx, c + 1000)).or_insert_with(Vec::new).push((x, y, dx, dy, c));

        let (mut nx, mut ny, mut nc) = (x + dx, y + dy, c + 1);
        while input[(nx, ny)] != b'#' {
            let (px, py, pc) = (nx - dx, ny - dy, nc - 1);
            prev.entry((nx, ny, dx, dy, nc)).or_insert_with(Vec::new).push((px, py, dx, dy, pc));
            queue.push((Reverse(nc), (nx, ny), (dx, dy)));
            (nx, ny, nc) = (nx + dx, ny + dy, nc + 1);
        }
    }

    let mut seen = HashSet::new();
    let mut best_seen = HashSet::new();
    let mut best_queue = Vec::new();

    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let k = (input.w() as isize - 2, 1, dx, dy, best_c.unwrap());
        if prev.contains_key(&k) {
            best_queue.push(k);
        }
    }

    while let Some((x, y, dx, dy, c)) = best_queue.pop() {
        if !best_seen.insert((x, y, dx, dy)) {
            continue;
        }
        seen.insert((x, y));
        if let Some(prev) = prev.get(&(x, y, dx, dy, c)) {
            for &(x, y, dx, dy, c) in prev {
                best_queue.push((x, y, dx, dy, c));
            }
        }
    }

    seen.len()
}
