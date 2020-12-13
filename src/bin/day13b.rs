use std::unreachable;

const INPUT: &str = include_str!("../../inputs/day13.txt");

#[derive(Copy, Clone, Debug)]
struct Bus {
    offset: u64,
    id: u64,
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn merge_buses(a: Bus, b: Bus) -> Bus {
    if b.offset < a.offset {
        merge_buses(b, a)
    } else {
        let offset = (b.offset - a.offset) % a.id;
        for i in 0.. {
            if (i * b.id) % a.id == offset {
                let id = (a.id * b.id) / gcd(a.id, b.id);
                return Bus {
                    offset: id - ((i * b.id) - b.offset),
                    id,
                };
            }
        }
        unreachable!()
    }
}

fn main() {
    let mut line_iter = INPUT.lines();
    let _target: u64 = line_iter.next().unwrap().parse().unwrap();
    let bus = line_iter
        .next()
        .unwrap()
        .split(',')
        .map(|id| id.parse().ok())
        .enumerate()
        .filter_map(|(offset, maybe_id)| {
            maybe_id.map(|id| Bus {
                offset: offset as u64,
                id,
            })
        })
        .fold(Bus { offset: 0, id: 1 }, merge_buses);

    println!("{}", bus.id - bus.offset);
}
