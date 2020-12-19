pub fn solve(input: &str) {
    for l in input.lines() {
        let tokens = tokenize(l);
        let ast = parse(&tokens);
        println!("{:?}", ast);
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    LeftParen,
    RightParen,
    Plus,
    Times,
    Num(u32),
}

#[derive(Debug)]
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Num(u32),
}

fn parse(tokens: &[Token]) -> Expr {
    // go from the right
    let l = tokens.len();
    match tokens[l - 1] {
        Token::RightParen => {
            let i = find_first_left_paren(tokens).unwrap();
            let e = parse(&tokens[i + 1..l - 1]);
            return parse_op(&tokens[..i], e);
        }
        Token::Num(n) => {
            if l == 1 {
                return Expr::Num(n);
            } else {
                return parse_op(&tokens[..l - 1], Expr::Num(n));
            }
        }
        _ => panic!("unexpected token"),
    }
}

fn find_first_left_paren(t: &[Token]) -> Option<usize> {
    for i in 0..t.len() {
        if t[i] == Token::LeftParen {
            return Some(i);
        }
    }
    None
}

fn parse_op(tokens: &[Token], right: Expr) -> Expr {
    let l = tokens.len();
    if l == 0 {
        return right;
    }

    match tokens[l - 1] {
        Token::Plus => return Expr::Add(Box::new(parse(&tokens[..l - 1])), Box::new(right)),
        Token::Times => return Expr::Mul(Box::new(parse(&tokens[..l - 1])), Box::new(right)),
        _ => panic!("unexpected token"),
    }
}

fn evaluate(expr: Expr) -> u32 {
    0
}

fn tokenize(l: &str) -> Vec<Token> {
    let mut v = Vec::new();
    let mut it = l.chars();
    // consume 1 char every time
    while let Some(c) = it.next() {
        match c {
            // assume 1 char per number
            '0'..='9' => v.push(Token::Num(c.to_digit(10).unwrap())),
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
