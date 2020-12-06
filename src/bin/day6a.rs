use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day6.txt");

fn main() {
    let res: usize = INPUT
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|line| line.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();
    println!("{}", res);
}
