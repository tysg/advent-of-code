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
    let pattern = Regex::new(part_1_regex.as_str()).unwrap();
    let count = input[1].lines().filter(|l| pattern.is_match(l)).count();
    println!("{}", count);

    // part 2: https://github.com/Lakret/aoc2020/blob/master/src/d19.rs#L3-L147
    let rule42_non_capturing = get_regex(42, &rules);
    let capturing_rule42 = format!("({})", &rule42_non_capturing);
    let rule42 = Regex::new(&capturing_rule42).unwrap();
    let rule31_non_capturing = get_regex(31, &rules);
    let capturing_rule31 = format!("({})", &rule31_non_capturing);
    let rule31 = Regex::new(&capturing_rule31).unwrap();
    let starts_42_ends_31 = format!(
        r"^(?P<start_42>(?:{})+)(?P<end_31>(?:{})+)$",
        rule42_non_capturing, rule31_non_capturing
    );
    let start_and_end31 = Regex::new(&starts_42_ends_31).unwrap();
    let matching = input[1]
        .lines()
        .filter(|message| matches_new_rules(&start_and_end31, &rule31, &rule42, message))
        .count();
    println!("{}", matching);
}

// https://github.com/Lakret/aoc2020/blob/master/src/d19.rs#L3-L147
fn matches_new_rules(
    starts_42_ends_31: &Regex,
    rule31: &Regex,
    rule42: &Regex,
    message: &str,
) -> bool {
    if starts_42_ends_31.is_match(message) {
        let capture = starts_42_ends_31.captures_iter(message).collect::<Vec<_>>();

        if capture.len() == 1 {
            let capture = capture.first().unwrap();
            let count_31 = rule31.find_iter(&capture["end_31"]).count();
            let count_42 = rule42.find_iter(&capture["start_42"]).count();

            count_42 >= count_31 + 1
        } else {
            false
        }
    } else {
        false
    }
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
                .trim_end_matches("|") // remove due to fold; use `fold_first` when it stablises
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
