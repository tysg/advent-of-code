use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_number: u32 = args[1].parse().unwrap();
    match day_number {
        1 => day1(&args[2]),
        2 => day2(&args[2]),
        _ => println!("wrong day_number!"),
    }
}

fn day1(filename: &str) {
    let content = fs::read_to_string(filename).expect("Something wrong reading file");
    let numbers: Vec<u32> = content
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    match find_pair_sums_to(&numbers, 2020) {
        Some(result) => println!("{}", result.0 * result.1),
        None => println!("no result"),
    }

    match three_sum(&numbers, 2020) {
        Some(result) => println!("{}", result.0 * result.1 * result.2),
        None => println!("no result"),
    }
}

fn parse_int(num: &str) -> u32 {
    num.parse::<u32>().unwrap()
}

fn day2(filename: &str) {
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

fn find_pair_sums_to(arr: &[u32], target: u32) -> Option<(u32, u32)> {
    let mut hash: HashMap<u32, usize> = HashMap::new();

    for (i, num) in arr.iter().enumerate() {
        if num > &target {
            continue;
        }
        match hash.get(num) {
            Some(&index) => return Some((*num, arr[index])),
            None => hash.insert(target - num, i),
        };
    }
    None
}

fn three_sum(arr: &[u32], target: u32) -> Option<(u32, u32, u32)> {
    for (i, num) in arr.iter().enumerate() {
        let res = find_pair_sums_to(&arr[i..], target - num);
        match res {
            Some((a, b)) => return Some((*num, a, b)),
            None => continue,
        };
    }
    None
}
