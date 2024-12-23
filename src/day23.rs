#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<([u8; 2], [u8; 2])>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once('-').unwrap();
            let l = l.as_bytes().try_into().unwrap();
            let r = r.as_bytes().try_into().unwrap();
            (l, r)
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut edges = FxHashMap::default();
    for &(l, r) in input {
        if l < r {
            edges.entry(l).or_insert_with(FxHashSet::default).insert(r);
        } else {
            edges.entry(r).or_insert_with(FxHashSet::default).insert(l);
        }
    }

    let mut count = 0;

    for (n1, edges1) in &edges {
        for n2 in edges1 {
            let Some(edges2) = edges.get(n2) else { continue };
            for n3 in edges2 {
                if n1[0] == b't' || n2[0] == b't' || n3[0] == b't' {
                    if edges1.contains(n3) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn gather_rec(
    edges: &FxHashMap<[u8; 2], FxHashSet<[u8; 2]>>,
    candidates: &[[u8; 2]],
    set: &mut FxHashSet<[u8; 2]>,
    best: &mut FxHashSet<[u8; 2]>,
) {
    if set.len() + candidates.len() <= best.len() {
        return;
    }

    for (i, &c) in candidates.iter().enumerate() {
        set.insert(c);

        let c_edges = &edges[&c];
        let candidates =
            candidates[i + 1..].iter().copied().filter(|c| c_edges.contains(c)).collect::<Vec<_>>();
        gather_rec(edges, &candidates, set, best);

        set.remove(&c);
    }

    if set.len() > best.len() {
        best.clone_from(&set);
    }
}

pub fn part2(input: &Input) -> String {
    let mut edges = FxHashMap::default();
    for &(l, r) in input {
        edges.entry(l).or_insert_with(FxHashSet::default).insert(r);
        edges.entry(r).or_insert_with(FxHashSet::default).insert(l);
    }

    let mut best = FxHashSet::default();
    let mut curr = FxHashSet::default();

    for &l in edges.keys() {
        curr.insert(l);
        let candidates = edges[&l].iter().copied().filter(|&v| v > l).sorted().collect::<Vec<_>>();
        gather_rec(&edges, &candidates, &mut curr, &mut best);
        curr.remove(&l);
    }

    best.into_iter().sorted().map(|[b1, b2]| format!("{}{}", b1 as char, b2 as char)).join(",")
}
