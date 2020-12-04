use regex::Regex;
use std::fs;
use crate::util::parse_int;

pub fn solve(filename: &str) {
    let content = fs::read_to_string(filename).expect("Something wrong reading file");
    let mut counter = 0;
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    for line in content.lines() {
        if let Some(cap) = re.captures(line) {
            // ignoring unmatched lines
            let low = parse_int(&cap.get(1).unwrap().as_str());
            let high = parse_int(&cap.get(2).unwrap().as_str());
            let c = &cap.get(3).unwrap().as_str().chars().nth(0).unwrap();
            let cand = &cap.get(4).unwrap().as_str();

            if check_follow_real_policy(low, high, *c, &cand) {
                counter += 1;
            }
        }
    }

    println!("{}", counter);
}

fn _check_follow_policy(lower_bound: u32, upper_bound: u32, c: char, candidate: &str) -> bool {
    let mut counter = 0;
    for cand in candidate.chars() {
        if &cand == &c {
            counter += 1;
        }
    }
    (counter >= lower_bound) && (counter <= upper_bound)
}

fn check_follow_real_policy(first_pos: u32, second_pos: u32, c: char, candidate: &str) -> bool {
    let chars = candidate.as_bytes();
    let first_char = chars[(first_pos - 1) as usize] as char;
    let second_char = chars[(second_pos - 1) as usize] as char;

    (first_char == c) ^ (second_char == c)
}

