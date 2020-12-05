const INPUT: &str = include_str!("../../inputs/day5.txt");

fn to_seat_id(s: &str) -> u32 {
    let mut v = 0;
    for c in s.chars() {
        v <<= 1;
        match c {
            'B' | 'R' => v |= 1,
            'F' | 'L' => {}
            _ => unreachable!(),
        }
    }
    v
}

fn main() {
    let res = INPUT.lines().map(to_seat_id).max().unwrap();
    println!("{}", res);
}
