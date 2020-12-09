const INPUT: &str = include_str!("../../inputs/day9.txt");

const TARGET: i64 = 217430975;

fn main() {
    let mut sum = 0;
    let mut count = 0;
    let values: Vec<i64> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    let mut it = values.iter().copied();

    for v in values.iter().copied() {
        sum += v;
        count += 1;
        while sum > TARGET {
            sum -= it.next().unwrap();
            count -= 1;
        }
        if sum == TARGET {
            let it_min = it.clone().take(count);
            let it_max = it_min.clone();
            println!("{}", it_max.max().unwrap() + it_min.min().unwrap());
            break;
        }
    }
}
