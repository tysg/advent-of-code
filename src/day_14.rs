use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq)]
enum Instr {
    Mask(u64, u64),
    Write(u64, u64),
    MaskV2(u64, VecDeque<u64>),
}

pub fn solve(input: &str) {
    // part 1
    let mut mem = HashMap::new();
    let mut write_mask = 0;
    let mut wipe_mask = 0;

    input
        .lines()
        .map(parse_instruction_1)
        .for_each(|i| match i {
            Instr::Mask(wipe, write) => {
                write_mask = write;
                wipe_mask = wipe;
            }
            Instr::Write(loc, n) => {
                mem.insert(loc, (n & wipe_mask) | write_mask);
            }
            _ => (), // safely igonored v2
        });

    let sum: u64 = mem.values().sum();
    println!("{}", sum);

    // part 2
    let mut mem = HashMap::new();
    let mut write_masks = VecDeque::new();
    let mut wipe_mask = 0;

    input
        .lines()
        .map(parse_instruction_2)
        .for_each(|i| match i {
            Instr::MaskV2(wipe, write) => {
                write_masks = write;
                wipe_mask = wipe;
            }
            Instr::Write(loc, n) => {
                &write_masks.iter().cloned().for_each(|m| {
                    mem.insert((loc & wipe_mask) | m, n);
                });
            }
            _ => (), // safely igonored v1
        });

    let sum: u64 = mem.values().sum();
    println!("{}", sum);
}

fn parse_instruction_1(input: &str) -> Instr {
    match &input[0..3] {
        "mem" => parse_write(input),
        "mas" => parse_mask(&input[7..]),
        _ => panic!("unexpected token"),
    }
}

fn parse_instruction_2(input: &str) -> Instr {
    match &input[0..3] {
        "mem" => parse_write(input),
        "mas" => parse_mask_v2(&input[7..]),
        _ => panic!("unexpected token"),
    }
}

fn parse_write(input: &str) -> Instr {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    }

    let caps = RE.captures(input).unwrap();
    let loc = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
    let content = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();

    Instr::Write(loc, content)
}

fn get_wipe_mask(mask: &str) -> u64 {
    let mut m = 0;
    for c in mask.chars() {
        m = m << 1;
        match c {
            '1' | '0' => m = m ^ 1,
            'X' => (),
            _ => panic!("unexpected token"),
        }
    }
    m
}

fn get_write_masks(mask: &str) -> VecDeque<u64> {
    let mut q = VecDeque::new();
    q.push_back(0);

    for c in mask.chars() {
        match c {
            '0' => {
                for _ in 0..q.len() {
                    let n = q.pop_front().unwrap();
                    q.push_back(n << 1);
                }
            }
            '1' => {
                for _ in 0..q.len() {
                    let n = q.pop_front().unwrap();
                    q.push_back((n << 1) | 1);
                }
            }
            'X' => {
                for _ in 0..q.len() {
                    let n = q.pop_front().unwrap();
                    q.push_back(n << 1);
                    q.push_back((n << 1) | 1);
                }
            }
            _ => panic!("unexpected token"),
        }
    }
    q
}

fn parse_mask_v2(mask: &str) -> Instr {
    Instr::MaskV2(get_wipe_mask(mask), get_write_masks(mask))
}

fn parse_mask(mask: &str) -> Instr {
    let mut and_mask = 0;
    let mut or_mask = 0;
    let mut it = mask.chars();
    for _ in 0..mask.len() {
        and_mask = and_mask << 1;
        or_mask = or_mask << 1;
        match it.next().unwrap() {
            '0' => {}
            '1' => {
                or_mask = or_mask | 1;
            }
            'X' => {
                and_mask = and_mask | 1;
            }
            _ => (),
        }
    }
    Instr::Mask(and_mask, or_mask)
}
