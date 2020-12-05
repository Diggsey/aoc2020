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
    let mut seats: Vec<_> = INPUT.lines().map(to_seat_id).collect();
    seats.sort();
    let gap = seats
        .windows(2)
        .find(|pair| pair[0] + 1 != pair[1])
        .unwrap();
    println!("{}", gap[0] + 1);
}
