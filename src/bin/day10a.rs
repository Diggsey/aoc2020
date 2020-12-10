const INPUT: &str = include_str!("../../inputs/day10.txt");

fn main() {
    let mut values: Vec<usize> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    values.sort();
    let mut prev = 0;
    let mut counts = [0, 0, 0, 1];
    for value in values {
        counts[value - prev] += 1;
        prev = value;
    }
    println!("{}", counts[1] * counts[3]);
}
