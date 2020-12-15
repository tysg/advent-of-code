use std::collections::HashMap;

pub fn solve(input: &str) {
    let arr = input
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut m: HashMap<usize, (Option<usize>, Option<usize>)> = arr
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, (None, Some(i))))
        .collect();

    let mut n = arr[arr.len() - 1];
    for turn in arr.len()..30000000 {
        match m.get(&n).unwrap() {
            (None, _) => n = 0,
            (Some(x), Some(y)) => n = y - x,
            (Some(_), None) => panic!("this shouldn't happen"),
        }

        let new_n_val = match m.get(&n) {
            None => (None, Some(turn)),
            Some(t) => (t.1, Some(turn)),
        };

        m.insert(n, new_n_val);
    }
    println!("{}", n);
}
