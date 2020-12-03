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

    match three_sum(&numbers, 2020) {
        Some(result) => println!("{}", result.0 * result.1 * result.2),
        None => println!("no result"),
    }



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
            Some((a,b)) => return Some((*num, a, b)),
            None => continue
        };
    }
    None
}



