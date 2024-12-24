#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = (HashMap<&'a str, bool>, HashMap<&'a str, (&'a str, &'a str, &'a str)>);

pub fn input_generator(input: &str) -> Input {
    let (init, ops) = input.split_once("\n\n").unwrap();
    let init = init
        .lines()
        .map(|line| {
            let (name, bit) = line.split_once(": ").unwrap();
            (name, bit == "1")
        })
        .collect();
    let ops = ops
        .lines()
        .map(|line| {
            let (arg1, rest) = line.split_once(' ').unwrap();
            let (op, rest) = rest.split_once(' ').unwrap();
            let (arg2, res) = rest.split_once(" -> ").unwrap();
            (res, (arg1, op, arg2))
        })
        .collect();
    (init, ops)
}

pub fn part1(input: &Input) -> usize {
    fn solve<'a>(
        n: &'a str,
        ops: &HashMap<&'a str, (&'a str, &'a str, &'a str)>,
        cache: &mut HashMap<&'a str, bool>,
    ) -> bool {
        if let Some(&res) = cache.get(n) {
            return res;
        }

        let (l, op, r) = ops[n];
        let l = solve(l, ops, cache);
        let r = solve(r, ops, cache);
        let res = match op {
            "XOR" => l ^ r,
            "OR" => l | r,
            "AND" => l & r,
            _ => unreachable!(),
        };

        cache.insert(n, res);

        res
    }

    let (init, ops) = input;
    let mut cache = init.clone();

    let mut acc = 0;
    for n in 0..=45 {
        let var = format!("z{n:02}");
        let (&k, _) = ops.get_key_value(&*var).unwrap();
        let b = solve(k, ops, &mut cache);
        acc |= (b as usize) << n;
    }
    acc
}

pub fn part2(input: &Input) -> String {
    fn solve<'a>(
        n: &'a str,
        ops: &HashMap<&'a str, (&'a str, &'a str, &'a str)>,
        cache: &mut HashMap<&'a str, bool>,
    ) -> bool {
        if let Some(&res) = cache.get(n) {
            return res;
        }

        let (l, op, r) = ops[n];
        let l = solve(l, ops, cache);
        let r = solve(r, ops, cache);
        let res = match op {
            "XOR" => l ^ r,
            "OR" => l | r,
            "AND" => l & r,
            _ => unreachable!(),
        };

        cache.insert(n, res);

        res
    }

    let (init, ops) = input;

    for i in 0..45 {
        let mut cache = HashMap::new();
        for j in 0..45 {
            let var = format!("x{j:02}");
            let (&k, _) = init.get_key_value(&*var).unwrap();
            cache.insert(k, false);

            let var = format!("y{j:02}");
            let (&k, _) = init.get_key_value(&*var).unwrap();
            cache.insert(k, i == j);
        }

        let mut acc = 0;
        for n in 0..=45 {
            let var = format!("z{n:02}");
            let (&k, _) = ops.get_key_value(&*var).unwrap();
            let b = solve(k, ops, &mut cache);
            acc |= (b as usize) << n;
        }

        println!("{i:>2}: {acc:046b}");
    }

    "TODO".to_string()
}
