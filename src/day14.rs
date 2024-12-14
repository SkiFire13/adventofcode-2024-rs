#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(i64, i64, i64, i64)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (p, v) = line[2..].split_once(" v=").unwrap();
            let (px, py) = p.split_once(',').unwrap();
            let (vx, vy) = v.split_once(',').unwrap();
            let px = px.parse().unwrap();
            let py = py.parse().unwrap();
            let vx = vx.parse().unwrap();
            let vy = vy.parse().unwrap();
            (px, py, vx, vy)
        })
        .collect()
}

const W: i64 = 101;
const H: i64 = 103;

pub fn part1(input: &Input) -> usize {
    let mut counts = [[0; 2]; 2];
    for &(px, py, vx, vy) in input {
        let fx = (px + 100 * vx).rem_euclid(W);
        let fy = (py + 100 * vy).rem_euclid(H);

        if fx != W / 2 && fy != H / 2 {
            counts[(fx < W / 2) as usize][(fy < H / 2) as usize] += 1;
        }
    }
    counts[0][0] * counts[0][1] * counts[1][0] * counts[1][1]
}

pub fn part2(input: &Input) -> usize {
    let mut positions = input.iter().map(|&(px, py, ..)| (px, py)).collect::<Vec<_>>();
    let mut i = 0;
    loop {
        i += 1;

        let mut xcounts = [0; W as usize];
        let mut ycounts = [0; H as usize];

        for ((px, py), &(.., vx, vy)) in std::iter::zip(&mut positions, input) {
            *px = (*px + vx).rem_euclid(W);
            *py = (*py + vy).rem_euclid(H);

            xcounts[*px as usize] += 1;
            ycounts[*py as usize] += 1;
        }

        if xcounts.iter().any(|&c| c > 20) && ycounts.iter().any(|&c| c > 20) {
            break i;
        }
    }
}
