use std::collections::HashSet;
use std::mem;

const INPUT: &str = include_str!("../../inputs/day17.txt");

#[derive(Clone, Debug)]
struct Map {
    cells: HashSet<(i32, i32, i32)>,
    min: (i32, i32, i32),
    max: (i32, i32, i32),
}

impl Map {
    fn recalculate_bounds(&mut self) {
        self.min.0 = self.cells.iter().map(|k| k.0).min().unwrap_or_default();
        self.min.1 = self.cells.iter().map(|k| k.1).min().unwrap_or_default();
        self.min.2 = self.cells.iter().map(|k| k.2).min().unwrap_or_default();
        self.max.0 = self.cells.iter().map(|k| k.0).max().unwrap_or_default();
        self.max.1 = self.cells.iter().map(|k| k.1).max().unwrap_or_default();
        self.max.2 = self.cells.iter().map(|k| k.2).max().unwrap_or_default();
    }
    fn new(cells: HashSet<(i32, i32, i32)>) -> Self {
        let mut res = Self {
            cells,
            min: (0, 0, 0),
            max: (0, 0, 0),
        };
        res.recalculate_bounds();
        res
    }
}

fn step(src: &Map, dst: &mut Map) {
    dst.cells.clear();

    for x in src.min.0 - 1..=src.max.0 + 1 {
        for y in src.min.1 - 1..=src.max.1 + 1 {
            for z in src.min.2 - 1..=src.max.2 + 1 {
                let live = src.cells.contains(&(x, y, z));
                let mut count = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        for dz in -1..=1 {
                            if dx == 0 && dy == 0 && dz == 0 {
                                continue;
                            }
                            if src.cells.contains(&(x + dx, y + dy, z + dz)) {
                                count += 1;
                            }
                        }
                    }
                }
                if count == 3 || (live && count == 2) {
                    dst.cells.insert((x, y, z));
                }
            }
        }
    }
    dst.recalculate_bounds();
}

fn main() {
    let mut map = Map::new(
        INPUT
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '#')
                    .map(move |(y, _)| (x as i32, y as i32, 0))
            })
            .collect(),
    );
    let mut map2 = map.clone();

    for _ in 0..6 {
        step(&map, &mut map2);
        mem::swap(&mut map, &mut map2);
    }

    println!("{}", map.cells.len());
}
