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
    let mut edges = HashMap::new();
    for &(l, r) in input {
        edges.entry(l).or_insert_with(HashSet::new).insert(r);
        edges.entry(r).or_insert_with(HashSet::new).insert(l);
    }

    let mut triples = HashSet::new();
    for &l in edges.keys() {
        for &r in &edges[&l] {
            if l < r {
                for &x in &edges[&r] {
                    if r < x {
                        if edges[&x].contains(&l) {
                            if l[0] == b't' || r[0] == b't' || x[0] == b't' {
                                triples.insert([l, r, x]);
                            }
                        }
                    }
                }
            }
        }
    }

    triples.len()
}

fn gather_rec(
    edges: &BTreeMap<[u8; 2], HashSet<[u8; 2]>>,
    set: &mut HashSet<[u8; 2]>,
    best: &mut HashSet<[u8; 2]>,
    min: [u8; 2],
) {
    let Some((&k, _)) =
        edges.iter().find(|(&k, v)| k > min && !set.contains(&k) && set.is_subset(v))
    else {
        if set.len() > best.len() {
            best.clone_from(&set);
        }
        return;
    };

    set.insert(k);
    gather_rec(edges, set, best, k);
    set.remove(&k);
}

pub fn part2(input: &Input) -> String {
    let mut edges = BTreeMap::new();
    for &(l, r) in input {
        edges.entry(l).or_insert_with(HashSet::new).insert(r);
        edges.entry(r).or_insert_with(HashSet::new).insert(l);
    }

    let mut best = HashSet::new();
    let mut curr = HashSet::new();

    for &l in edges.keys() {
        curr.insert(l);
        gather_rec(&edges, &mut curr, &mut best, l);
        curr.remove(&l);
    }

    best.into_iter().sorted().map(|[b1, b2]| format!("{}{}", b1 as char, b2 as char)).join(",")
}
