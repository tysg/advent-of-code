#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    LeftParen,
    RightParen,
    Plus,
    Times,
    Num(u64),
}

fn tokenize(l: &str) -> Vec<Token> {
    let mut v = Vec::new();
    let mut it = l.chars();
    // consume 1 char every time
    while let Some(c) = it.next() {
        match c {
            // assume 1 char per number
            '0'..='9' => v.push(Token::Num(c.to_digit(10).unwrap() as u64)),
            '(' => v.push(Token::LeftParen),
            ')' => v.push(Token::RightParen),
            '+' => v.push(Token::Plus),
            '*' => v.push(Token::Times),
            ' ' => (),
            _ => panic!("unexpected token"),
        }
    }

    v
}

pub fn solve(input: &str) {
    // the easy way
    let sum: u64 = input.lines().map(|l| eval(tokenize(l))).sum();
    println!("{}", sum);
}

fn eval(l: Vec<Token>) -> u64 {
    let mut s = Vec::new();
    let mut it = l.iter();

    while let Some(c) = it.next() {
        match c {
            Token::Num(_) | Token::Times | Token::Plus | Token::LeftParen => {
                s.push(*c);
            }

            Token::RightParen => {
                // pop until next '('
                let mut local = Vec::new();
                while s.len() > 0 {
                    let last = s[s.len() - 1];
                    if last == Token::LeftParen {
                        s.pop();
                        break;
                    } else {
                        s.pop();
                        local.push(last);
                    }
                }

                let rev = local.iter().rev().cloned().collect::<Vec<_>>();
                let res = eval_base_level_add_first(rev);
                s.push(Token::Num(res));
            }
        }
    }

    eval_base_level_add_first(s)
}

fn eval_base_level_add_first(s: Vec<Token>) -> u64 {
    let mut local = Vec::new();

    for t in s.iter() {
        match t {
            Token::Num(n) => {
                if local.len() > 0 {
                    match local[local.len() - 1] {
                        Token::Plus => {
                            local.pop(); // +
                            if let Token::Num(y) = local.pop().unwrap() {
                                local.push(Token::Num(n + y));
                            }
                        }
                        Token::Times => {
                            local.push(*t);
                        }
                        _ => panic!("unexpected token"),
                    }
                } else {
                    local.push(*t);
                }
            }
            _ => {
                local.push(*t);
            }
        }
    }

    let mut product = 1;
    for t in local.iter() {
        match t {
            Token::Num(n) => {
                product *= n;
            }
            _ => (),
        }
    }
    product
}

fn _eval_base_level(s: Vec<Token>) -> u64 {
    let mut sum = 0;
    let mut is_add = true;
    for t in s.iter() {
        match t {
            Token::Plus => {
                is_add = true;
            }
            Token::Num(n) => {
                if is_add {
                    sum += n;
                } else {
                    sum *= n;
                }
            }
            Token::Times => {
                is_add = false;
            }
            _ => panic!("unexpected token"),
        }
    }

    sum
}
