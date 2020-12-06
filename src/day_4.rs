use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum LengthUnit {
    Cm,
    In,
}

#[derive(Debug)]
struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<(u32, LengthUnit)>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn parse_int_if(input: &str, check: fn(u32) -> bool) -> Option<u32> {
    if let Ok(x) = input.parse::<u32>() {
        if check(x) {
            return Some(x);
        }
    }
    None
}

fn parse_height(input: &str) -> Option<(u32, LengthUnit)> {
    let len = input.len();
    let unit = &input[len - 2..len];
    let val = &input[..len - 2];

    match unit {
        "cm" => parse_int_if(val, |n| n >= 150 && n <= 193).map(|n| (n, LengthUnit::Cm)),
        "in" => parse_int_if(val, |n| n >= 59 && n <= 76).map(|n| (n, LengthUnit::In)),
        _ => None,
    }
}

impl From<&str> for Passport {
    fn from(input: &str) -> Passport {
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
            static ref ECL_RE: Regex = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
            static ref PID_RE: Regex = Regex::new(r"[0-9]{9}").unwrap();
        }
        let mut it = input.split_whitespace();
        let mut p = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };
        while let Some(field) = it.next() {
            let key = &field[..3];
            let val = &field[4..];
            match key {
                "byr" => {
                    p.byr = if val.len() != 4 {
                        None
                    } else {
                        parse_int_if(val, |n| n >= 1920 && n <= 2002)
                    }
                }
                "iyr" => {
                    p.iyr = if val.len() != 4 {
                        None
                    } else {
                        parse_int_if(val, |n| n >= 2010 && n <= 2020)
                    }
                }
                "eyr" => {
                    p.eyr = if val.len() != 4 {
                        None
                    } else {
                        parse_int_if(val, |n| n >= 2020 && n <= 2030)
                    }
                }
                "hgt" => p.hgt = parse_height(val),
                "hcl" => {
                    p.hcl = if HCL_RE.is_match(val) {
                        Some(val.to_string())
                    } else {
                        None
                    }
                }
                "ecl" => {
                    p.ecl = if ECL_RE.is_match(val) {
                        Some(val.to_string())
                    } else {
                        None
                    }
                }
                "pid" => {
                    p.pid = if PID_RE.is_match(val) {
                        Some(val.to_string())
                    } else {
                        None
                    }
                }
                _ => (),
            }
        }
        p
    }
}

fn rule_north_pole(x: &Passport) -> bool {
    x.byr.is_some()
        && x.iyr.is_some()
        && x.eyr.is_some()
        && x.hgt.is_some()
        && x.hcl.is_some()
        && x.ecl.is_some()
        && x.pid.is_some()
}

pub fn solve(content: String) {
    let valid_passports = content
        .split("\n\n")
        .map(Passport::from)
        .filter(rule_north_pole)
        .count();
    println!("{:?}", valid_passports);
}
