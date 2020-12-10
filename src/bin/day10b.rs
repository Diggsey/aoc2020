const INPUT: &str = include_str!("../../inputs/day10.txt");

fn main() {
    let mut values: Vec<usize> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    values.push(0);
    values.sort();

    let mut counts: Vec<u64> = Vec::with_capacity(values.len());
    counts.push(1);
    for i in 1..values.len() {
        let v = values[i];
        let mut count = 0;
        for j in (0..i).rev() {
            if v - values[j] > 3 {
                break;
            }
            count += counts[j];
        }
        counts.push(count);
    }
    println!("{}", counts.last().unwrap());
}
