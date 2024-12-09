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
    let mut poss = Vec::new();
    let mut heaps = [const { BinaryHeap::new() }; 10];

    let mut pos = 0;
    for i in 0..input.len() {
        poss.push(pos);
        pos += input[i] as usize;
        if i % 2 == 1 {
            heaps[input[i] as usize].push(Reverse(i));
        }
    }

    let mut tot = 0;

    for i in (0..(input.len() + 1) / 2).rev() {
        let i = 2 * i;
        let len = input[i] as usize;

        let mut min_l = 0;
        let mut min_j = i;
        for l in len..10 {
            if let Some(&Reverse(j)) = heaps[l].peek() {
                if j < min_j {
                    (min_l, min_j) = (l, j);
                }
            }
        }

        if min_l != 0 {
            heaps[min_l].pop().unwrap();
            let pos = poss[min_j];
            let new_pos = pos + len;
            tot += (i / 2) * ((new_pos * (new_pos - 1) / 2) - (pos * (pos - 1) / 2));
            poss[min_j] += len;
            let new_len = min_l - len;
            if new_len != 0 {
                heaps[new_len].push(Reverse(min_j));
            }
        } else {
            let pos = poss[i];
            let new_pos = pos + len;
            tot += (i / 2) * ((new_pos * (new_pos - 1) / 2) - (pos * (pos.wrapping_sub(1)) / 2));
        }
    }

    tot
}
