use std::env;
mod day_1;
mod day_2;
mod day_3;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_number: u32 = args[1].parse().unwrap();
    match day_number {
        1 => day_1::solve(&args[2]),
        2 => day_2::solve(&args[2]),
        3 => day_3::solve(&args[2]),
        _ => println!("wrong day_number!"),
    }
}

