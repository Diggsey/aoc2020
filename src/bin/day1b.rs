const INPUT: &str = include_str!("../../inputs/day1.txt");

fn check(values: &[i64], remaining: usize, total: i64, product: i64) {
    if remaining == 0 {
        if total == 0 {
            println!("{}", product);
        }
    } else {
        for offset in 0..=(values.len() - remaining) {
            check(
                &values[offset..],
                remaining - 1,
                total - values[offset],
                product * values[offset],
            );
        }
    }
}

fn main() {
    let values: Vec<i64> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
    check(values.as_slice(), 3, 2020, 1)
}
