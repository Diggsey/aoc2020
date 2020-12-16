use std::collections::HashSet;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../inputs/day16.txt");

struct Rule {
    name: &'static str,
    ranges: Vec<RangeInclusive<u32>>,
}

impl Rule {
    fn check(&self, value: u32) -> bool {
        self.ranges.iter().any(|range| range.contains(&value))
    }
}

fn solve(
    possible_rules: &[Vec<usize>],
    result: &mut Vec<usize>,
    visited: &mut HashSet<usize>,
) -> bool {
    if possible_rules.is_empty() {
        true
    } else {
        for &v in &possible_rules[0] {
            if visited.contains(&v) {
                continue;
            }
            result.push(v);
            visited.insert(v);
            if solve(&possible_rules[1..], result, visited) {
                return true;
            }
            visited.remove(&v);
            result.pop();
        }
        false
    }
}

fn parse_ticket(line: &'static str) -> Vec<u32> {
    line.split(",").map(|p| p.parse::<u32>().unwrap()).collect()
}

fn main() {
    let mut it = INPUT.split("\n\n");
    let rules: Vec<_> = it
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut rule_it = line.splitn(2, ": ");
            let name = rule_it.next().unwrap();
            let ranges: Vec<_> = rule_it
                .next()
                .unwrap()
                .split(" or ")
                .map(|rule_part| {
                    let mut part_it = rule_part.splitn(2, "-").map(|p| p.parse::<u32>().unwrap());
                    part_it.next().unwrap()..=part_it.next().unwrap()
                })
                .collect();
            Rule { name, ranges }
        })
        .collect();

    let my_ticket = parse_ticket(it.next().unwrap().lines().nth(1).unwrap());
    let tickets: Vec<_> = it
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(parse_ticket)
        .filter(|ticket| {
            ticket
                .iter()
                .all(|&p| rules.iter().any(|rule| rule.check(p)))
        })
        .collect();

    let possible_rules: Vec<_> = (0..tickets[0].len())
        .map(|i| {
            rules
                .iter()
                .enumerate()
                .filter(|(_, rule)| {
                    tickets
                        .iter()
                        .map(|ticket| ticket[i])
                        .all(|p| rule.check(p))
                })
                .map(|(j, _)| j)
                .collect::<Vec<_>>()
        })
        .collect();

    let mut visited = HashSet::new();
    let mut result = Vec::new();
    solve(&possible_rules, &mut result, &mut visited);

    let res: u64 = my_ticket
        .iter()
        .copied()
        .zip(result)
        .filter(|&(_, i)| rules[i].name.starts_with("departure "))
        .map(|(p, _)| p as u64)
        .product();
    println!("{}", res);
}
