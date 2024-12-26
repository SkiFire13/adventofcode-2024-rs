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

    // Initially we always have:
    //   x00 XOR y00 => z00
    //   x00 AND z00 => carry01
    // This allows us to get the carry for the next full adder,
    // which we will continuously update.
    let (_, mut carry) = oops[&(Var::from_n('x', 0), Op::And)];

    for n in 1..45 {
        // For every full adder we should have:
        //   xNN XOR yNN => res
        //   xNN AND yNN => carry_1
        //   res XOR carryNN => zNN
        //   res AND carryNN => carry_2
        //   carry_1 OR carry_2 => carryNN+1
        //
        // 2 of the 5 outputs could be swapped, so we can't rely on them.
        // However we know that the inputs are always the same, so we can
        // base ourselves on them to get the true values for each role
        // and compare them with the actual ones.
        //
        // We thus actually see the following:
        //   xNN XOR yNN => got_res
        //   xNN AND yNN => got_carry_1
        //   true_res XOR carryNN => got_z
        //   true_res AND carryNN => got_carry_2
        //   true_carry_1 OR true_carry_2 => got_carry
        //
        // We know what `xNN`, `yNN` and `carryNN` are, so we can find `got_res`,
        // `got_carry_1`, `true_res`, `got_z` and `got_carry_2`. Moreover we also
        // know the true value for `got_z`, which is `zNN`.
        // We can now compare `true_res` with `got_res` and `zNN` with `got_z`
        // to determine which ones of them are wrong.
        //
        // Moreover we know that one of `got_carry_1` and `got_carry_2` is correct,
        // since swapping them changes nothing. Hence we can use that to determine
        // what the true value for the other and what `got_carry` is.
        // This way we can also determine whether `got_carry_1` or `got_carry_2` is wrong.
        //
        // Finally, since at most 2 values can be swapped, and hence wrong:
        // - if none of these 4 are wrong, they must all be right;
        // - if exactly one of these 4 is wrong, the other wrong one must be `got_carry`
        //   and the true `carry_NN+1` must be the first wrong one.
        // - if two of these 4 are wrong, they must be swapped. `got_carry` must then be correct.
        //
        // This way we have determined which pair of gate outputs must be swapped, if any,
        // and what is the true value of `carryNN+1`, which is needed for the next iteration.

        let x = Var::from_n('x', n);
        let z = Var::from_n('z', n);
        let (_, got_carry_1) = oops[&(x, Op::And)];
        let (_, got_res) = oops[&(x, Op::Xor)];
        let (true_res, got_carry_2) = oops[&(carry, Op::And)];
        let (_, got_z) = oops[&(carry, Op::Xor)];

        // Check the value we got for `zNN` is equal to the true one
        if got_z != z {
            out.push(got_z);
        }

        // Check the value we got for `res` is equal to the true one
        if got_res != true_res {
            out.push(got_res);
        }

        // Either get the actual value for `carryNN` or determine that the
        // actual value for `carry_1` is wrong.
        if let Some(&(_, new_carry)) = oops.get(&(got_carry_1, Op::Or)) {
            carry = new_carry
        } else {
            out.push(got_carry_1);
        }

        // Either get the actual value for `carryNN` or determine that the
        // actual value for `carry_2` is wrong.
        if let Some(&(_, new_carry)) = oops.get(&(got_carry_2, Op::Or)) {
            carry = new_carry
        } else {
            out.push(got_carry_2);
        }

        // At this point the variable `carry` holds the actual value for `carryNN+1`,
        // since at least one of the two `if let Some` must have succeeded due to
        // at least one of the two `carry_{1,2}` being correct.

        // `out` always has an even length after each iteration. If the length
        // is odd it means we found exactly one wrong output in the previous checks.
        // In that case the actual `carryNN+1` is wrong and its real value is
        // the other wrong value identified, that is the last value of `out`.
        if out.len() % 2 == 1 {
            out.push(carry);
            carry = out[out.len() - 2];
        }
    }

    out.iter().sorted().map(|v| v.as_str()).join(",")
}
