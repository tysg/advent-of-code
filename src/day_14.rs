use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Instr {
    Mask(u64, u64),
    Write(usize, u64),
}

pub fn solve(input: &str) {
    let mut mem = HashMap::new();
    let mut and_mask = 0;
    let mut or_mask = 0;

    input.lines().map(parse_instruction).for_each(|i| {
        match i {
            Instr::Mask(and, or) => {
                and_mask = and;
                or_mask = or;
            }
            Instr::Write(loc, n) => {
                mem.insert(loc, n & and_mask ^ or_mask);
            }
        }
    });

    let sum: u64 = mem.values().sum();
    println!("{}", sum);
}

fn parse_instruction(input: &str) -> Instr {
    match &input[0..3] {
        "mem" => parse_write(input),
        "mas" => parse_mask(input),
        _ => panic!("unexpected token"),
    }
}

fn parse_write(input: &str) -> Instr {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    }

    let caps = RE.captures(input).unwrap();
    let loc = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let content = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();

    Instr::Write(loc, content)
}

fn parse_mask(input: &str) -> Instr {
    let mut and_mask = 0;
    let mut or_mask = 0;
    let mask = &input[7..];
    let mut it = mask.chars();
    for _ in 0..mask.len() {
        and_mask = and_mask << 1;
        or_mask = or_mask << 1;
        match it.next().unwrap() {
            '0' => {}
            '1' => {
                or_mask = or_mask ^ 1;
            }
            'X' => {
                and_mask = and_mask ^ 1;
            }
            _ => (),
        }
    }
    Instr::Mask(and_mask, or_mask)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mask_1() {
        let ms = "mask = 01X01111101010010X01101X0110101111X0";
        let and = 0b001000000000000001000001000000000010;
        let or = 0b010011111010100100011010011010111100;
        assert_eq!(parse_mask(ms), Instr::Mask(and, or));
    }
}
