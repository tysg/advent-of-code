use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Range {
    name: String,
    r1: (usize, usize),
    r2: (usize, usize),
}

pub fn solve(input: &str) {
    let mut it = input.split("\n\n");

    let ranges = it.next().unwrap();
    let mut my_ticket = it.next().unwrap().lines().skip(1);
    let nearby_tickets = it.next().unwrap().lines().skip(1);

    let ranges = ranges.lines().map(parse_range).collect::<Vec<_>>();
    let my_ticket = parse_ticket(my_ticket.next().unwrap());
    let nearby_tickets = nearby_tickets.map(parse_ticket).collect::<Vec<_>>();

    // part 1
    let scanning_error_rate = get_scanning_error_rate(&nearby_tickets, &ranges);
    println!("{}", scanning_error_rate);

    // part 2
    let nearby_tickets = nearby_tickets
        .iter()
        .filter(|ticket| {
            !ticket
                .iter()
                .map(|v| test_ranges(v, &ranges))
                .any(|v| v.is_some())
        })
        .collect::<Vec<_>>();
    let len = ranges.len();
    assert_eq!(ranges.len(), my_ticket.len());
    let mut domains = vec![(0..len).collect::<HashSet<_>>(); len];

    for ticket in nearby_tickets {
        for field_ix in 0..len {
            for range_ix in 0..len {
                if !test_single_range(&ticket[field_ix], &ranges[range_ix]) {
                    domains[field_ix].remove(&range_ix);
                }
            }
        }

        // check if we reached desired state
        if domains.iter().all(|s| s.len() == 1) {
            println!("We entered the desired state");
            break;
        }
    }

    let assignment = assign(&domains);

    let my_multiplication: usize = (0..6)
        .map(|n| assignment.iter().enumerate().find(|t| t.1 == &n).unwrap().0)
        .map(|id| my_ticket[id])
        .product();
    println!("{:?}", assignment);

    println!("{}", my_multiplication);
}

fn assign(domain: &Vec<HashSet<usize>>) -> Vec<usize> {
    let mut domain = domain.clone();
    let len = domain.len();
    let mut assignment = vec![0; len];
    // brute force, expensive, may not terminate
    while domain.iter().any(|s| s.len() > 0) {
        for i in 0..len {
            if domain[i].len() == 1 {
                assignment[i] = *domain[i].iter().next().unwrap();

                for j in 0..len {
                    domain[j].remove(&assignment[i]);
                }
            }
        }
    }

    assignment
}

fn get_scanning_error_rate(nearby_tickets: &Vec<Vec<usize>>, ranges: &Vec<Range>) -> usize {
    nearby_tickets
        .iter()
        .map(|v| {
            v.iter()
                .filter_map(|field| test_ranges(field, &ranges))
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn test_ranges(field: &usize, ranges: &Vec<Range>) -> Option<usize> {
    for r in ranges {
        if test_single_range(field, r) {
            return None;
        }
    }
    Some(*field)
}

fn test_single_range(field: &usize, range: &Range) -> bool {
    (range.r1.0..=range.r1.1).contains(field) || (range.r2.0..=range.r2.1).contains(field)
}

fn parse_ticket(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_range(input: &str) -> Range {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z\s]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    }
    let caps = RE.captures(input).unwrap();
    let name = caps.get(1).unwrap().as_str().to_string();
    let r1_start = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let r1_end = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
    let r2_start = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
    let r2_end = caps.get(5).unwrap().as_str().parse::<usize>().unwrap();
    Range {
        name,
        r1: (r1_start, r1_end),
        r2: (r2_start, r2_end),
    }
}
