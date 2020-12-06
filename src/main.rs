use std::{env, fs};
use std::io::{self, Read};
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod util;

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("Something wrong reading file")
}

fn read_stdin() -> String {
    let mut out = String::new();
    io::stdin().read_to_string(&mut out)
        .expect("Something wrong reading from stdin");
    out
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let day_number: u32 = args[1].parse().unwrap();

    let is_reading_stdin = args.len() == 2;

    let input = if is_reading_stdin{
        read_stdin()
    } else {
        read_file(&args[2])
    };

    match day_number {
        1 => day_1::solve(input),
        2 => day_2::solve(input),
        3 => day_3::solve(input),
        4 => day_4::solve(input),
        5 => day_5::solve(input),
        _ => println!("wrong day_number!"),
    }
}

