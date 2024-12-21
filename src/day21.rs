#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<Vec<Pad9>>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'A' => Pad9::A,
                    '0' => Pad9::N0,
                    '1' => Pad9::N1,
                    '2' => Pad9::N2,
                    '3' => Pad9::N3,
                    '4' => Pad9::N4,
                    '5' => Pad9::N5,
                    '6' => Pad9::N6,
                    '7' => Pad9::N7,
                    '8' => Pad9::N8,
                    '9' => Pad9::N9,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pad {
    L,
    R,
    U,
    D,
    A,
}

impl Pad {
    pub fn mov(self, action: Pad) -> Option<(Self, Option<Pad>)> {
        let pos = match (self, action) {
            (_, Pad::A) => return Some((self, Some(self))),
            (Pad::L, Pad::R) => Pad::D,
            (Pad::R, Pad::L) => Pad::D,
            (Pad::R, Pad::U) => Pad::A,
            (Pad::U, Pad::R) => Pad::A,
            (Pad::U, Pad::D) => Pad::D,
            (Pad::D, Pad::L) => Pad::L,
            (Pad::D, Pad::R) => Pad::R,
            (Pad::D, Pad::U) => Pad::U,
            (Pad::A, Pad::L) => Pad::U,
            (Pad::A, Pad::D) => Pad::R,
            _ => return None,
        };
        Some((pos, None))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pad9 {
    A,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
}

impl Pad9 {
    pub fn mov(self, action: Pad) -> Option<(Self, Option<Pad9>)> {
        use Pad9::*;
        let pos = match (self, action) {
            (_, Pad::A) => return Some((self, Some(self))),
            (A, Pad::L) => N0,
            (A, Pad::U) => N3,
            (N0, Pad::R) => A,
            (N0, Pad::U) => N2,
            (N1, Pad::R) => N2,
            (N1, Pad::U) => N4,
            (N2, Pad::L) => N1,
            (N2, Pad::R) => N3,
            (N2, Pad::U) => N5,
            (N2, Pad::D) => N0,
            (N3, Pad::L) => N2,
            (N3, Pad::U) => N6,
            (N3, Pad::D) => A,
            (N4, Pad::R) => N5,
            (N4, Pad::U) => N7,
            (N4, Pad::D) => N1,
            (N5, Pad::L) => N4,
            (N5, Pad::R) => N6,
            (N5, Pad::U) => N8,
            (N5, Pad::D) => N2,
            (N6, Pad::L) => N5,
            (N6, Pad::U) => N9,
            (N6, Pad::D) => N3,
            (N7, Pad::R) => N8,
            (N7, Pad::D) => N4,
            (N8, Pad::L) => N7,
            (N8, Pad::R) => N9,
            (N8, Pad::D) => N5,
            (N9, Pad::L) => N8,
            (N9, Pad::D) => N6,
            _ => return None,
        };
        Some((pos, None))
    }
}

fn pad_map(distances: impl Fn(Pad, Pad) -> usize) -> HashMap<(Pad, Pad), usize> {
    let mut map = HashMap::new();
    for start in [Pad::A, Pad::L, Pad::R, Pad::U, Pad::D] {
        for end in [Pad::A, Pad::L, Pad::R, Pad::U, Pad::D] {
            let mut queue = BinaryHeap::from([(Reverse(0), start, Pad::A)]);
            let mut min = usize::MAX;
            while let Some((Reverse(pressed), p2, p1)) = queue.pop() {
                if pressed >= min {
                    map.insert((start, end), min);
                    break;
                }
                for p2act in [Pad::A, Pad::L, Pad::R, Pad::U, Pad::D] {
                    let pressed = pressed + distances(p1, p2act);

                    let Some((p2, final_act)) = p2.mov(p2act) else { continue };
                    let Some(final_act) = final_act else {
                        queue.push((Reverse(pressed), p2, p2act));
                        continue;
                    };
                    if final_act == end {
                        min = min.min(pressed);
                    }
                }
            }
        }
    }
    map
}

pub fn part1(input: &Input) -> usize {
    let map1 = pad_map(|_, _| 1);
    let map2 = pad_map(|s, e| map1[&(s, e)]);

    input
        .iter()
        .map(|seq| {
            let mut n = 0;
            for p in seq {
                n = 10 * n
                    + match p {
                        Pad9::A => continue,
                        Pad9::N0 => 0,
                        Pad9::N1 => 1,
                        Pad9::N2 => 2,
                        Pad9::N3 => 3,
                        Pad9::N4 => 4,
                        Pad9::N5 => 5,
                        Pad9::N6 => 6,
                        Pad9::N7 => 7,
                        Pad9::N8 => 8,
                        Pad9::N9 => 9,
                    };
            }

            let c = std::iter::once(Pad9::A)
                .chain(seq.iter().copied())
                .tuple_windows()
                .map(|(start, goal)| {
                    let mut min = usize::MAX;
                    let mut queue = BinaryHeap::from([(Reverse(0), start, Pad::A)]);
                    while let Some((Reverse(pressed), p9, p1)) = queue.pop() {
                        if pressed >= min {
                            return min;
                        }
                        for p9act in [Pad::A, Pad::L, Pad::R, Pad::U, Pad::D] {
                            let pressed = pressed + map2[&(p1, p9act)];
                            let Some((p9, final_act)) = p9.mov(p9act) else { continue };
                            let Some(final_act) = final_act else {
                                queue.push((Reverse(pressed), p9, p9act));
                                continue;
                            };
                            if final_act == goal {
                                min = min.min(pressed);
                            }
                        }
                    }

                    unreachable!()
                })
                .sum::<usize>();
            n * c
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
    let mut i = 1;
    let mut map = pad_map(|_, _| 1);
    while i != 25 {
        map = pad_map(|s, e| map[&(s, e)]);
        i += 1;
    }

    input
        .iter()
        .map(|seq| {
            let mut n = 0;
            for p in seq {
                n = 10 * n
                    + match p {
                        Pad9::A => continue,
                        Pad9::N0 => 0,
                        Pad9::N1 => 1,
                        Pad9::N2 => 2,
                        Pad9::N3 => 3,
                        Pad9::N4 => 4,
                        Pad9::N5 => 5,
                        Pad9::N6 => 6,
                        Pad9::N7 => 7,
                        Pad9::N8 => 8,
                        Pad9::N9 => 9,
                    };
            }

            let c = std::iter::once(Pad9::A)
                .chain(seq.iter().copied())
                .tuple_windows()
                .map(|(start, goal)| {
                    let mut min = usize::MAX;
                    let mut queue = BinaryHeap::from([(Reverse(0), start, Pad::A)]);
                    while let Some((Reverse(pressed), p9, p1)) = queue.pop() {
                        if pressed >= min {
                            return min;
                        }
                        for p9act in [Pad::A, Pad::L, Pad::R, Pad::U, Pad::D] {
                            let pressed = pressed + map[&(p1, p9act)];
                            let Some((p9, final_act)) = p9.mov(p9act) else { continue };
                            let Some(final_act) = final_act else {
                                queue.push((Reverse(pressed), p9, p9act));
                                continue;
                            };
                            if final_act == goal {
                                min = min.min(pressed);
                            }
                        }
                    }

                    unreachable!()
                })
                .sum::<usize>();
            n * c
        })
        .sum()
}
