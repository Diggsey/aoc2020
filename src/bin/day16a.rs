use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../inputs/day16.txt");

struct Rule {
    #[allow(unused)]
    name: &'static str,
    ranges: Vec<RangeInclusive<u32>>,
}

impl Rule {
    fn check(&self, value: u32) -> bool {
        self.ranges.iter().any(|range| range.contains(&value))
    }
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

    let _ = it.next();
    let tickets: Vec<Vec<_>> = it
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| line.split(",").map(|p| p.parse::<u32>().unwrap()).collect())
        .collect();

    let error_rate: u32 = tickets
        .iter()
        .flatten()
        .copied()
        .filter(|&p| !rules.iter().any(|rule| rule.check(p)))
        .sum();

    println!("{}", error_rate);
}
