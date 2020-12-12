use std::cmp;
use std::collections::HashSet;
pub fn solve(input: &str) {
    let arr = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let wrong_num = find_wrong_num(&arr).unwrap();
    let weakness = find_encryption_weakness(&arr, wrong_num).unwrap();

    println!("Wrong num is: {}; Weakness is: {}", wrong_num, weakness);
}

fn find_encryption_weakness(arr: &Vec<i64>, target: i64) -> Option<i64> {
    let mut left = 0;
    let mut sum = 0;
    for right in 0..arr.len() {
        sum += arr[right];

        while sum > target && left <= right {
            sum -= arr[left];
            left += 1;
        }
        if sum == target && left != right {
            return Some(max_min_sum(&arr[left..=right]));
        }
    }

    None
}

fn max_min_sum(arr: &[i64]) -> i64 {
    let mut max = 0;
    let mut min = arr[0];
    for n in arr {
        max = cmp::max(max, *n);
        min = cmp::min(min, *n);
    }

    max + min
}

fn find_wrong_num(arr: &Vec<i64>) -> Option<i64> {
    let windows_size = 25;
    let mut s = arr.iter().take(windows_size).collect::<HashSet<_>>();

    for i in windows_size..arr.len() {
        if let None = s.iter().find(|n| s.contains(&(&arr[i] - **n))) {
            return Some(arr[i]);
        }
        s.remove(&arr[i - windows_size]);
        s.insert(&arr[i]);
    }
    None
}
