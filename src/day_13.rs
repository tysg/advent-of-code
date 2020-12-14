use crate::util::chinese_remainder;
pub fn solve(input: &str) {
    let mut it = input.lines();
    let earliest = it.next().unwrap().parse::<u64>().unwrap();
    let schedule = it
        .next()
        .unwrap()
        .split(",")
        .map(parse_schedule)
        .collect::<Vec<_>>();

    let part1 = schedule
        .iter()
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .map(|n| (n, n - earliest % n))
        .min_by(|x, y| (x.1).cmp(&y.1))
        .unwrap();

    println!("{}", part1.0 as u64 * part1.1);

    // part 2
    // find n such that a[i] - n % a[i] = i
    // or, n % a[i] = a[i] - i

    let (modulii, residues): (Vec<i64>, Vec<i64>) = schedule
        .iter()
        .enumerate()
        .filter(|o| o.1.is_some())
        .map(|(i, v)| (v.unwrap() as i64, v.unwrap() as i64 - i as i64))
        .unzip();

    match chinese_remainder(&residues, &modulii) {
        Some(n) => println!("{}", n),
        None => println!("Not found"),
    }
}

fn parse_schedule(i: &str) -> Option<u64> {
    match i {
        "x" => None,
        num => Some(num.parse().unwrap()),
    }
}
