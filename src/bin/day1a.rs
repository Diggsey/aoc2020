use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    let values: HashSet<i32> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
    for &v in &values {
        let u = 2020 - v;
        if values.contains(&u) {
            println!("{}, {}, {}", u, v, u * v);
        }
    }
}
