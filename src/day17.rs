#[allow(unused_imports)]
use super::prelude::*;
type Input = ([i64; 3], Vec<u8>);

pub fn input_generator(input: &str) -> Input {
    let (regs, prog) = input.split_once("\n\n").unwrap();
    let (reg_a, rest) = regs.split_once("\n").unwrap();
    let (reg_b, reg_c) = rest.split_once("\n").unwrap();
    let (_, reg_a) = reg_a.split_once(": ").unwrap();
    let (_, reg_b) = reg_b.split_once(": ").unwrap();
    let (_, reg_c) = reg_c.split_once(": ").unwrap();
    let reg_a = reg_a.parse().unwrap();
    let reg_b = reg_b.parse().unwrap();
    let reg_c = reg_c.parse().unwrap();
    let (_, prog) = prog.split_once(": ").unwrap();
    let prog = prog.trim().split(",").map(|n| n.parse().unwrap()).collect();
    ([reg_a, reg_b, reg_c], prog)
}

fn simulate(mut regs: [i64; 3], prog: &[u8], out: &mut Vec<u8>) {
    let mut ip = 0;
    while ip < prog.len() {
        let instr = prog[ip];
        let op = prog[ip + 1];
        let combo_op = || match op {
            0..=3 => op,
            4..=6 => regs[op as usize - 4] as u8,
            _ => unreachable!(),
        };
        match instr {
            0 => regs[0] >>= combo_op(),
            1 => regs[1] ^= op as i64,
            2 => regs[1] = combo_op() as i64 & 0b111,
            3 => {
                if regs[0] != 0 {
                    ip = op as usize;
                    continue;
                }
            }
            4 => regs[1] ^= regs[2],
            5 => out.push(combo_op() & 0b111),
            6 => regs[1] = regs[0] >> combo_op(),
            7 => regs[2] = regs[0] >> combo_op(),
            _ => unreachable!(),
        }
        ip += 2;
    }
}

pub fn part1(input: &Input) -> String {
    let &(regs, ref prog) = input;
    let mut out = Vec::new();
    simulate(regs, prog, &mut out);
    out.iter().join(",")
}

pub fn part2(input: &Input) -> i64 {
    let &(_, ref prog) = input;

    let mut new_a = 0;
    let mut out = Vec::new();

    loop {
        out.clear();
        simulate([new_a, 0, 0], prog, &mut out);

        if prog.ends_with(&out) {
            if prog.len() == out.len() {
                return new_a;
            } else {
                new_a <<= 3;
            }
        } else {
            while new_a & 0b111 == 0b111 {
                new_a >>= 3;
            }
            new_a += 1;
        }
    }
}
