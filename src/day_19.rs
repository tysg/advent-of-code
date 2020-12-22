use regex::Regex;
use std::collections::HashMap;

enum Rule {
    Node(Vec<Vec<usize>>),
    Leaf(String),
}
pub fn solve(input: &str) {
    let input = input.split("\n\n").collect::<Vec<_>>();
    let rules = input[0]
        .lines()
        .map(parse_rule)
        .collect::<HashMap<usize, Rule>>();
    let part_1_regex = add_full_match_carets(get_regex(0, &rules));
    // println!("{}", part_1_regex);
    let pattern = Regex::new(part_1_regex.as_str()).unwrap();
    let count = input[1].lines().filter(|l| pattern.is_match(l)).count();
    println!("{}", count);
}

fn add_full_match_carets(pat: String) -> String {
    let mut s = String::new();
    s.push('^');
    s.push_str(pat.as_str());
    s.push('$');
    s
}

fn get_regex(i: usize, m: &HashMap<usize, Rule>) -> String {
    match m.get(&i).unwrap() {
        Rule::Leaf(s) => return s.to_string(),
        Rule::Node(children) => {
            let mut res = children
                .iter()
                .map(|child| {
                    child
                        .iter()
                        .map(|i| get_regex(*i, m))
                        .fold(String::new(), |mut acc, x| {
                            acc.push_str(x.as_str());
                            acc
                        })
                })
                .fold(String::from("("), |mut acc, x| {
                    acc.push_str(x.as_str());
                    acc.push('|');
                    acc
                })
                .trim_end_matches("|") // due to fold, use `fold_first` when it stablises
                .to_string();
            res.push(')');
            res
        }
    }
}

fn parse_rule(line: &str) -> (usize, Rule) {
    let mut it = line.split(" ").peekable();
    let index = it.next().unwrap();
    let index = index[..index.len() - 1].parse::<usize>().unwrap();

    let peek = it.peek().unwrap();

    if &peek[..1] == "\"" {
        // a leaf
        let leaf = it.next().unwrap();
        return (index, Rule::Leaf(leaf[1..leaf.len() - 1].to_string()));
    }

    let mut children = Vec::new();

    while it.peek().is_some() {
        let mut child = Vec::new();

        while let Some(token) = it.next() {
            match token {
                "|" => break,
                _ => child.push(token.parse::<usize>().unwrap()),
            }
        }
        children.push(child);
    }

    (index, Rule::Node(children))
}
