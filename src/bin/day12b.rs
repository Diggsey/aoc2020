const INPUT: &str = include_str!("../../inputs/day12.txt");

fn main() {
    let mut pos = (0, 0);
    let mut dir = (10, -1);

    for line in INPUT.lines() {
        let c = &line[0..1];
        let value: i32 = line[1..].parse().unwrap();

        match (c, value) {
            ("L", 0) | ("R", 0) | ("L", 360) | ("R", 360) => {}
            ("L", 90) | ("R", 270) => dir = (dir.1, -dir.0),
            ("L", 180) | ("R", 180) => dir = (-dir.0, -dir.1),
            ("L", 270) | ("R", 90) => dir = (-dir.1, dir.0),
            ("F", value) => pos = (pos.0 + dir.0 * value, pos.1 + dir.1 * value),
            ("N", value) => dir.1 -= value,
            ("E", value) => dir.0 += value,
            ("S", value) => dir.1 += value,
            ("W", value) => dir.0 -= value,
            _ => unimplemented!(),
        }
    }

    println!("{}", pos.0.abs() + pos.1.abs());
}
