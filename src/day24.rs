#[allow(unused_imports)]
use super::prelude::*;
type Input<'a> = (HashMap<Var, bool>, HashMap<Var, (Var, Op, Var)>);

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var([u8; 3]);

impl Var {
    fn from_str(s: &str) -> Self {
        Self(s.as_bytes().try_into().unwrap())
    }
    fn from_n(c: char, n: usize) -> Self {
        let n = n as u8;
        Self([c as u8, b'0' + n / 10, b'0' + n % 10])
    }
    fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Op {
    And,
    Xor,
    Or,
}

pub fn input_generator(input: &str) -> Input {
    let (init, ops) = input.split_once("\n\n").unwrap();
    let init = init
        .lines()
        .map(|line| {
            let (name, bit) = line.split_once(": ").unwrap();
            (Var::from_str(name), bit == "1")
        })
        .collect();
    let ops = ops
        .lines()
        .map(|line| {
            let (arg1, rest) = line.split_once(' ').unwrap();
            let (op, rest) = rest.split_once(' ').unwrap();
            let (arg2, res) = rest.split_once(" -> ").unwrap();
            let res = Var::from_str(res);
            let arg1 = Var::from_str(arg1);
            let arg2 = Var::from_str(arg2);
            let op = match op {
                "AND" => Op::And,
                "XOR" => Op::Xor,
                "OR" => Op::Or,
                _ => unreachable!(),
            };
            (res, (arg1, op, arg2))
        })
        .collect();
    (init, ops)
}

pub fn part1(input: &Input) -> usize {
    fn compute_rec<'a>(
        n: Var,
        ops: &HashMap<Var, (Var, Op, Var)>,
        cache: &mut HashMap<Var, bool>,
    ) -> bool {
        if let Some(&res) = cache.get(&n) {
            return res;
        }

        let (l, op, r) = ops[&n];
        let l = compute_rec(l, ops, cache);
        let r = compute_rec(r, ops, cache);
        let res = match op {
            Op::And => l & r,
            Op::Xor => l ^ r,
            Op::Or => l | r,
        };

        cache.insert(n, res);

        res
    }

    let (init, ops) = input;
    let mut cache = init.clone();

    let mut acc = 0;
    for n in 0..=45 {
        let var = Var::from_n('z', n);
        let b = compute_rec(var, ops, &mut cache);
        acc |= (b as usize) << n;
    }
    acc
}

pub fn part2(input: &Input) -> String {
    let (_, ops) = input;

    let mut oops = HashMap::new();
    for (&out, &(l, op, r)) in ops {
        oops.insert((l, op), (r, out));
        oops.insert((r, op), (l, out));
    }

    let mut out = Vec::new();

    let (_, mut carry) = oops[&(Var::from_n('x', 0), Op::And)];
    for n in 1..45 {
        let x = Var::from_n('x', n);
        let z = Var::from_n('z', n);
        let (_, act_carry_1) = oops[&(x, Op::And)];
        let (_, act_res) = oops[&(x, Op::Xor)];
        let (exp_res, act_carry_2) = oops[&(carry, Op::And)];
        let (_, act_z) = oops[&(carry, Op::Xor)];

        if act_z.0[0] != b'z' {
            out.extend([act_z, z]);

            (_, carry) =
                *oops.get(&(act_carry_1, Op::Or)).unwrap_or_else(|| &oops[&(act_carry_2, Op::Or)]);

            if carry == z {
                carry = act_z;
            }
        } else {
            if act_res != exp_res {
                out.push(act_res);
            }

            if let Some(&(_, new_carry)) = oops.get(&(act_carry_1, Op::Or)) {
                carry = new_carry
            } else {
                out.push(act_carry_1);
            }

            if let Some(&(_, new_carry)) = oops.get(&(act_carry_2, Op::Or)) {
                carry = new_carry
            } else {
                out.push(act_carry_2);
            }

            if out.len() % 2 == 1 {
                out.push(carry);
                carry = out[out.len() - 2];
            }
        }
    }

    out.iter().sorted().map(|v| v.as_str()).join(",")
}
