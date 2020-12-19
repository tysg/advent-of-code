use std::io::{self, Read};
use std::{env, fs};
mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18a;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

mod util;

fn read_file(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}

fn read_stdin() -> Result<String, io::Error> {
    let mut out = String::new();
    io::stdin().read_to_string(&mut out)?;
    Ok(out)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough parameters");
        return;
    }

    let day_number: u32 = match args[1].parse() {
        Ok(n) => n,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let is_reading_stdin = args.len() == 2;

    let input = if is_reading_stdin {
        read_stdin()
    } else {
        read_file(&args[2])
    };

    let input = match input {
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let input = input.trim();

    match day_number {
        1 => day_1::solve(input),
        2 => day_2::solve(input),
        3 => day_3::solve(input),
        4 => day_4::solve(input),
        5 => day_5::solve(input),
        6 => day_6::solve(input),
        7 => day_7::solve(input),
        8 => day_8::solve(input),
        9 => day_9::solve(input),
        10 => day_10::solve(input),
        11 => day_11::solve(input),
        12 => day_12::solve(input),
        13 => day_13::solve(input),
        14 => day_14::solve(input),
        15 => day_15::solve(input),
        16 => day_16::solve(input),
        17 => day_17::solve(input),
        18 => day_18a::solve(input),
        _ => println!("wrong day_number!"),
    }
}
