pub fn solve(input: String) {
    let mut ids = input
        .lines()
        .map(process_boarding_pass)
        .collect::<Vec<u32>>();

    ids.sort();

    let my_seat = bin_search_missing(&ids);

    match my_seat {
        Some(n) => println!("{}", n),
        None => println!("Max id not found?"),
    }
}

fn bin_search_missing(ids: &[u32]) -> Option<u32> {
    let offset = ids[0];

    let mut left = 0;
    let mut right = ids.len() - 1;

    while left <= right {
        let mid = left + (right - left / 2);
        if ids[mid] == offset + mid as u32 {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    if left == ids.len() {
        None
    } else {
        Some(ids[left] - 1)
    }
}

fn _search_missing(ids: &[u32]) -> Option<u32> {
    // linear search the first missing int
    for i in 1..ids.len() {
        if ids[i] != (ids[i - 1] + 1) {
            return Some(ids[i - 1] + 1);
        }
    }

    None
}

fn process_boarding_pass(input: &str) -> u32 {
    let mut row = 0b0;
    let mut col = 0b0;
    let mut it = input.chars();

    for _ in 0..7 {
        row = row << 1;
        match it.next().unwrap() {
            'B' => row = row | 0b1,
            'F' => (),
            _ => panic!("this char shouldn't be here"),
        }
    }

    for _ in 0..3 {
        col = col << 1;
        match it.next().unwrap() {
            'R' => col = col | 0b1,
            'L' => (),
            _ => panic!("this char shouldn't be here"),
        }
    }

    (row << 3) + col
}
