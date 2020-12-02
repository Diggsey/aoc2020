use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let mut total = 0;
    for line in INPUT.lines() {
        let (a, b, c, pw) =
            scan_fmt!(line, "{d}-{d} {/./}: {}", usize, usize, char, String).unwrap();
        let n = pw
            .chars()
            .enumerate()
            .filter(|&(i, d)| (d == c) && (i + 1 == a || i + 1 == b))
            .count();
        if n == 1 {
            total += 1;
        }
    }
    println!("{}", total);
}
