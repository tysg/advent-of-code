use std::collections::{HashMap, VecDeque};
type Rule = (String, Vec<(String, u32)>);

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

    // bfs
    let mut q: VecDeque<usize> = VecDeque::new();
    let init = bags_index.get("shiny gold").unwrap();
    q.push_back(*init);
    let mut visited = vec![false; len];

    while let Some(node) = q.pop_front() {

        for i in 0..len {
            if g[i][node] != 0 && !visited[i] {
                visited[i] = true;
                q.push_back(i);
            }
        }
    }


    let bag_contains_shiny_gold = visited.iter().filter(|x| **x).count();
    println!("{}", bag_contains_shiny_gold);
}

fn parse(input: &str) -> Vec<Rule> {
    input.lines().map(parse_rule).collect()
}

fn parse_rule(l: &str) -> Rule {
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

        if let Some(c) = it.next(){
            if c.as_bytes()[c.len()-1] == '.' as u8 {
                break;
            }
        }
    }

    (parent, children)
}
