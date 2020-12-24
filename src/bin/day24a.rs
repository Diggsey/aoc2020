use std::collections::HashSet;
use std::str::FromStr;
use std::unreachable;

const INPUT: &str = include_str!("../../inputs/day24.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    a: i32,
    b: i32,
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut c_it = s.chars();
        let mut a = 0;
        let mut b = 0;
        while let Some(c) = c_it.next() {
            match c {
                'e' => a += 1,
                'w' => a -= 1,
                'n' => match c_it.next().unwrap() {
                    'w' => b -= 1,
                    'e' => {
                        a += 1;
                        b -= 1;
                    }
                    _ => unreachable!(),
                },
                's' => match c_it.next().unwrap() {
                    'e' => b += 1,
                    'w' => {
                        b += 1;
                        a -= 1;
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        Ok(Self { a, b })
    }
}

fn main() {
    let mut black_tiles = HashSet::new();
    for line in INPUT.lines() {
        let coord: Coord = line.parse().unwrap();
        if !black_tiles.remove(&coord) {
            black_tiles.insert(coord);
        }
    }
    println!("{}", black_tiles.len());
}
