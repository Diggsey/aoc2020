use std::collections::HashSet;
use std::mem;
use std::str::FromStr;

const INPUT: &str = include_str!("../../inputs/day24.txt");

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    a: i32,
    b: i32,
}

impl Coord {
    fn neighbours(&self) -> [Coord; 6] {
        [
            Coord {
                a: self.a - 1,
                b: self.b,
            },
            Coord {
                a: self.a + 1,
                b: self.b,
            },
            Coord {
                a: self.a,
                b: self.b - 1,
            },
            Coord {
                a: self.a,
                b: self.b + 1,
            },
            Coord {
                a: self.a - 1,
                b: self.b + 1,
            },
            Coord {
                a: self.a + 1,
                b: self.b - 1,
            },
        ]
    }
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

fn step(src: &HashSet<Coord>, dst: &mut HashSet<Coord>) {
    dst.clear();
    let min_a = src.iter().map(|coord| coord.a).min().unwrap() - 1;
    let min_b = src.iter().map(|coord| coord.b).min().unwrap() - 1;
    let max_a = src.iter().map(|coord| coord.a).max().unwrap() + 1;
    let max_b = src.iter().map(|coord| coord.b).max().unwrap() + 1;

    for a in min_a..=max_a {
        for b in min_b..=max_b {
            let coord = Coord { a, b };
            let neighbours = coord
                .neighbours()
                .iter()
                .copied()
                .filter(|c| src.contains(c))
                .count();
            if neighbours == 2 || (neighbours == 1 && src.contains(&coord)) {
                dst.insert(coord);
            }
        }
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
    let mut black_tiles2 = black_tiles.clone();

    for _ in 0..100 {
        step(&black_tiles, &mut black_tiles2);
        mem::swap(&mut black_tiles, &mut black_tiles2);
    }

    println!("{}", black_tiles.len());
}
