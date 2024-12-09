#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<u8>;

pub fn input_generator(input: &str) -> Input {
    input.trim().bytes().map(|b| b - b'0').collect()
}

pub fn part1(input: &Input) -> usize {
    let mut idf = 0;
    let mut idb = input.len() - 1;
    let mut idb_len = input[idb] as usize;

    assert!(idb % 2 == 0);

    let mut tot = 0;
    let mut pos = 0;

    while idf < idb {
        let len = input[idf] as usize;
        let new_pos = pos + len;
        tot += (idf / 2) * ((new_pos * (new_pos - 1) / 2) - (pos * (pos.wrapping_sub(1)) / 2));
        pos = new_pos;

        idf += 1;
        if idf < idb {
            let mut fill_len = input[idf] as usize;

            while fill_len != 0 && idf < idb {
                let len = std::cmp::min(fill_len, idb_len);
                let new_pos = pos + len;
                tot += (idb / 2) * ((new_pos * (new_pos - 1) / 2) - (pos * (pos - 1) / 2));
                pos = new_pos;
                idb_len -= len;
                fill_len -= len;
                if idb_len == 0 {
                    idb -= 2;
                    idb_len = input[idb] as usize;
                }
            }

            idf += 1;
        }
    }

    let new_pos = pos + idb_len;
    tot += (idb / 2) * ((new_pos * (new_pos - 1) / 2) - (pos * (pos - 1) / 2));

    tot
}

pub fn part2(input: &Input) -> usize {
    let mut pos = 0;
    let mut poss = Vec::new();
    for i in 0..input.len() {
        poss.push(pos);
        pos += input[i] as usize;
    }

    let mut tot = 0;

    for i in (0..(input.len() + 1) / 2).rev() {
        if let Some(j) =
            (0..i).find(|&j| poss[2 + 2 * j] - poss[1 + 2 * j] >= input[2 * i] as usize)
        {
            let pos = poss[1 + 2 * j];
            let len = input[2 * i] as usize;
            let new_pos = pos + len;
            tot += i * ((new_pos * (new_pos - 1) / 2) - (pos * (pos - 1) / 2));
            poss[1 + 2 * j] += len;
        } else {
            let pos = poss[2 * i];
            let len = input[2 * i] as usize;
            let new_pos = pos + len;
            tot += i * ((new_pos * (new_pos - 1) / 2) - (pos * (pos.wrapping_sub(1)) / 2));
        }
    }

    tot
}
