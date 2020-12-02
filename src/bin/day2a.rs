use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day2.txt");

fn main() {
    let mut total = 0;
    for line in INPUT.lines() {
        let (a, b, c, pw) =
            scan_fmt!(line, "{d}-{d} {/./}: {}", usize, usize, char, String).unwrap();
        let n = pw.chars().filter(|&d| d == c).count();
        if n >= a && n <= b {
            total += 1;
        }
    }
    println!("{}", total);
}
