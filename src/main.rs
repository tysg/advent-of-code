use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let content = fs::read_to_string(filename).expect("Something wrong reading file");
    let numbers: Vec<u32> = content
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();
    match find_pair_sums_to(&numbers, 2020) {
        Some(result) => println!("{}", result.0 * result.1),
        None => println!("no result"),
    }
}

fn find_pair_sums_to(arr: &Vec<u32>, target: u32) -> Option<(u32, u32)> {
    let mut hash: HashMap<u32, usize> = HashMap::new();

    for (i, num) in arr.iter().enumerate() {
        match hash.get(num) {
            Some(&index) => return Some((num.clone(), arr[index])),
            None => hash.insert(target - num, i),
        };
    }
    None
}
