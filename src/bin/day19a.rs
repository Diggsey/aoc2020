use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day19.txt");

struct Sequence {
    children: Vec<i32>,
}

enum Rule {
    Leaf(char),
    Branch(Vec<Sequence>),
}

fn expand(mut rules: HashMap<i32, Rule>) -> HashSet<String> {
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
    expanded.remove(&0).unwrap()
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
        .collect();

    let expanded = expand(rules);
    let res = sections
        .next()
        .unwrap()
        .lines()
        .filter(|&input| expanded.contains(input))
        .count();

    println!("{}", res);
}
