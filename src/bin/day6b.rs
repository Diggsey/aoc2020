use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day6.txt");

fn main() {
    let res: usize = INPUT
        .split("\n\n")
        .map(|group| {
            let mut lines = group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>());
            let mut s = lines.next().unwrap();
            for line in lines {
                s = s.intersection(&line).copied().collect();
            }
            s.len()
        })
        .sum();
    println!("{}", res);
}
