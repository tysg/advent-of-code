mod parser;

use std::collections::{HashMap, VecDeque};
type Rule = (String, Vec<(String, u32)>);

struct BagsGraph<'a> {
    g: Vec<Vec<u32>>,
    m: HashMap<&'a str, usize>,
}

pub fn solve(input: &str) {
    let rules = parse(input);

    let bags_index: HashMap<&str, usize> = rules
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0.as_str(), i as usize))
        .collect();

    let len = rules.len();
    let mut g = vec![vec![0; len]; len];

    // construct graph
    for rule in &rules {
        let pi = bags_index.get(&rule.0.as_str()).unwrap();
        for c in &rule.1 {
            let ci = bags_index.get(&c.0.as_str()).unwrap();
            g[*pi][*ci] = c.1;
        }
    }

    let graph = BagsGraph {
        g: g,
        m: bags_index,
    };

    let init = graph.m.get("shiny gold").unwrap();

    let num_of_colors = count_colors(&graph, *init);
    println!("{}", num_of_colors);

    let num_of_required_bags = count_total_required_bags(&graph, *init) - 1;
    println!("{}", num_of_required_bags);
}

fn count_total_required_bags(g: &BagsGraph, color_index: usize) -> u64 {
    let mut count: u64 = 1; // including this bag

    for (i, child) in g.g[color_index].iter().enumerate() {
        if *child != 0 {
            count += (*child as u64) * count_total_required_bags(g, i);
        }
    }

    count
}

fn count_colors(g: &BagsGraph, init_index: usize) -> u32 {
    let len = g.g.len();

    // bfs
    let mut q: VecDeque<usize> = VecDeque::new();
    q.push_back(init_index);
    let mut visited = vec![false; len];

    while let Some(node) = q.pop_front() {
        for i in 0..len {
            if g.g[i][node] != 0 && !visited[i] {
                visited[i] = true;
                q.push_back(i);
            }
        }
    }

    visited.iter().filter(|x| **x).count() as u32
}

fn parse(input: &str) -> Vec<Rule> {
    input.lines().map(crate::day_7::parser::parse).collect()
}

fn _parse_rule(l: &str) -> Rule {
    let mut it = l.split_whitespace();
    let mut parent = String::new();
    let mut children = Vec::new();

    parent.push_str(it.next().unwrap());
    parent.push(' ');
    parent.push_str(it.next().unwrap());

    it.next(); // "bag contains"
    it.next();

    while let Some(n) = it.next() {
        if n == "no" {
            it.next();
            it.next();
            continue;
        }

        let num = n.parse::<u32>().unwrap();
        let mut color = String::new();
        color.push_str(it.next().unwrap());
        color.push(' ');
        color.push_str(it.next().unwrap());

        children.push((color, num));

        if let Some(c) = it.next() {
            if c.as_bytes()[c.len() - 1] == '.' as u8 {
                break;
            }
        }
    }

    (parent, children)
}
