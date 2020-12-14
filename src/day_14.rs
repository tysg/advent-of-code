use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq)]
enum Instr {
    // Mask(u64, u64),
    Write(u64, u64),
    MaskV2(String),
}

pub fn solve(input: &str) {
    let mut mem = HashMap::new();
    let mut masks = VecDeque::new();

    input.lines().map(parse_instruction).for_each(|i| match i {
        Instr::MaskV2(s) => {
            masks = get_masks(&s);
        }
        Instr::Write(loc, n) => {
            &masks.iter().cloned().for_each(|m| {
                mem.insert(loc ^ m, n);
            });
        }
    });

    let sum: u64 = mem.values().sum();
    println!("{}", sum);
}

fn parse_instruction(input: &str) -> Instr {
    match &input[0..3] {
        "mem" => parse_write(input),
        "mas" => parse_mask_v2(input),
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

fn get_masks(mask: &str) -> VecDeque<u64> {
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
                    q.push_back((n << 1) ^ 1);
                }
            }
            'X' => {
                for _ in 0..q.len() {
                    let n = q.pop_front().unwrap();
                    q.push_back(n << 1);
                    q.push_back((n << 1) ^ 1);
                }
            }
            _ => ()
        }
    }
    q
}

fn parse_mask_v2(input: &str) -> Instr {
    Instr::MaskV2(input[7..].to_string())
}

// fn _parse_mask(input: &str) -> Instr {
//     let mut and_mask = 0;
//     let mut or_mask = 0;
//     let mask = &input[7..];
//     let mut it = mask.chars();
//     for _ in 0..mask.len() {
//         and_mask = and_mask << 1;
//         or_mask = or_mask << 1;
//         match it.next().unwrap() {
//             '0' => {}
//             '1' => {
//                 or_mask = or_mask ^ 1;
//             }
//             'X' => {
//                 and_mask = and_mask ^ 1;
//             }
//             _ => (),
//         }
//     }
//     Instr::Mask(and_mask, or_mask)
// }

