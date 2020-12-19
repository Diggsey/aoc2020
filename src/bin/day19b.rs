use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day19.txt");

struct Sequence {
    children: Vec<i32>,
}

enum Rule {
    Leaf(char),
    Branch(Vec<Sequence>),
}

fn expand(mut rules: HashMap<i32, Rule>) -> HashMap<i32, HashSet<String>> {
    let unit_set: HashSet<String> = Some(String::new()).into_iter().collect();
    let mut expanded: HashMap<i32, HashSet<String>> = HashMap::new();
    while !rules.is_empty() {
        rules.retain(|&k, v| match v {
            Rule::Leaf(c) => {
                expanded.entry(k).or_default().insert(c.to_string());
                false
            }
            Rule::Branch(options) => {
                if options
                    .iter()
                    .flat_map(|seq| seq.children.iter())
                    .all(|i| expanded.contains_key(i))
                {
                    let mut res = HashSet::new();
                    for option in options {
                        let s = option.children.iter().fold(unit_set.clone(), |s, j| {
                            let mut res = HashSet::new();
                            for a in &s {
                                for b in &expanded[j] {
                                    res.insert(a.clone() + b);
                                }
                            }
                            res
                        });
                        for v in s {
                            res.insert(v);
                        }
                    }
                    expanded.insert(k, res);
                    false
                } else {
                    true
                }
            }
        })
    }
    expanded
}

fn parse_8_0(input: &str, expanded: &HashMap<i32, HashSet<String>>) -> bool {
    let s = &expanded[&42];
    for item in s {
        if let Some(remainder) = input.strip_prefix(item) {
            if parse_8_0(remainder, expanded) || parse_11_0(remainder, expanded, 0) {
                return true;
            }
        }
    }
    false
}

fn parse_11_0(input: &str, expanded: &HashMap<i32, HashSet<String>>, n: usize) -> bool {
    let s = &expanded[&42];
    for item in s {
        if let Some(remainder) = input.strip_prefix(item) {
            if parse_11_0(remainder, expanded, n + 1) || parse_11_1(remainder, expanded, n) {
                return true;
            }
        }
    }
    false
}

fn parse_11_1(input: &str, expanded: &HashMap<i32, HashSet<String>>, n: usize) -> bool {
    let s = &expanded[&31];
    for item in s {
        if let Some(remainder) = input.strip_prefix(item) {
            if n == 0 {
                if remainder.is_empty() {
                    return true;
                }
            } else if parse_11_1(remainder, expanded, n - 1) {
                return true;
            }
        }
    }
    false
}

fn main() {
    let mut sections = INPUT.split("\n\n");
    let rules: HashMap<i32, _> = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut lr_iter = line.splitn(2, ": ");
            let rule_index = lr_iter.next().unwrap().parse().unwrap();
            let rhs = lr_iter.next().unwrap();
            if let Some(s) = rhs.strip_prefix('"') {
                (rule_index, Rule::Leaf(s.chars().next().unwrap()))
            } else {
                let options = rhs
                    .split(" | ")
                    .map(|option| Sequence {
                        children: option.split(' ').map(|s| s.parse().unwrap()).collect(),
                    })
                    .collect();
                (rule_index, Rule::Branch(options))
            }
        })
        .filter(|&(i, _)| i != 8 && i != 11 && i != 0)
        .collect();

    let expanded = expand(rules);

    let res = sections
        .next()
        .unwrap()
        .lines()
        .filter(|&input| parse_8_0(input, &expanded))
        .count();

    println!("{}", res);
}
