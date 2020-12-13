const INPUT: &str = include_str!("../../inputs/day13.txt");

struct Bus {
    id: u64,
    wait_time: u64,
}

fn main() {
    let mut line_iter = INPUT.lines();
    let target: u64 = line_iter.next().unwrap().parse().unwrap();
    let bus = line_iter
        .next()
        .unwrap()
        .split(',')
        .filter(|&id| id != "x")
        .map(|id| id.parse().unwrap())
        .map(|id| Bus {
            id,
            wait_time: (id - 1) - ((target + id - 1) % id),
        })
        .min_by_key(|bus| bus.wait_time)
        .unwrap();

    println!("{}", bus.id * bus.wait_time);
}
