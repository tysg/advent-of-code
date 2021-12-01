use std::collections::HashSet;
pub fn solve(input: &str) {
    let sum: u32 = input.split("\n\n").map(process_group).sum();
    println!("{}", sum);
}

fn _process_group(input: &str) -> u32 {
    let empty_set = HashSet::new();
    let answers: HashSet<u8> = input
        .lines()
        .map(|l| l.as_bytes().iter().cloned().collect::<HashSet<u8>>())
        .fold(empty_set, |acc, x| acc.union(&x).cloned().collect());

    answers.len() as u32
}

fn process_group(input: &str) -> u32 {
    let all_letters = ('a' as u8..='z' as u8).collect::<HashSet<u8>>();
    let answers: HashSet<u8> = input
        .lines()
        .map(|l| l.as_bytes().iter().cloned().collect::<HashSet<u8>>())
        .fold(all_letters, |acc, x| {
            acc.intersection(&x).cloned().collect()
        });

    answers.len() as u32
}
