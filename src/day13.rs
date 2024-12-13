#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<((usize, usize), (usize, usize), (usize, usize))>;

pub fn input_generator(input: &str) -> Input {
    input
        .trim()
        .split("\n\n")
        .map(|block| {
            let (_, rest) = block.split_once(": X+").unwrap();
            let (ba, rest) = rest.split_once("\n").unwrap();
            let (_, rest) = rest.split_once(": X+").unwrap();
            let (bb, rest) = rest.split_once("\n").unwrap();
            let (_, rest) = rest.split_once(": X=").unwrap();
            let (dxa, dya) = ba.split_once(", Y+").unwrap();
            let (dxb, dyb) = bb.split_once(", Y+").unwrap();
            let (x, y) = rest.split_once(", Y=").unwrap();
            (
                (dxa.parse().unwrap(), dya.parse().unwrap()),
                (dxb.parse().unwrap(), dyb.parse().unwrap()),
                (x.parse().unwrap(), y.parse().unwrap()),
            )
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut tot = 0;
    for &((dxa, dya), (dxb, dyb), (x, y)) in input {
        let (dxa, dya, dxb, dyb, x, y) =
            (dxa as i64, dya as i64, dxb as i64, dyb as i64, x as i64, y as i64);
        let det = dxa * dyb - dxb * dya;
        let d1 = x * dyb - dxb * y;
        let d2 = dxa * y - x * dya;

        if d1 % det == 0 && d2 % det == 0 && d1 / det >= 0 && d2 / det >= 0 {
            tot += (3 * (d1 / det) + (d2 / det)) as usize;
        }
    }
    tot
}

pub fn part2(input: &Input) -> usize {
    let mut tot = 0;
    for &((dxa, dya), (dxb, dyb), (x, y)) in input {
        let (x, y) = (x + 10000000000000, y + 10000000000000);

        let (dxa, dya, dxb, dyb, x, y) =
            (dxa as i64, dya as i64, dxb as i64, dyb as i64, x as i64, y as i64);
        let det = dxa * dyb - dxb * dya;
        let d1 = x * dyb - dxb * y;
        let d2 = dxa * y - x * dya;

        if d1 % det == 0 && d2 % det == 0 && d1 / det >= 0 && d2 / det >= 0 {
            tot += (3 * (d1 / det) + (d2 / det)) as usize;
        }
    }
    tot
}
