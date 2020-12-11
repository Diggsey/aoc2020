use std::mem;

const INPUT: &str = include_str!("../../inputs/day11.txt");

#[derive(Copy, Clone, Debug, PartialEq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Cell {
    fn from(other: char) -> Self {
        match other {
            '.' => Cell::Floor,
            'L' => Cell::Empty,
            '#' => Cell::Occupied,
            _ => panic!(),
        }
    }
}

fn get(src: &[Vec<Cell>], mut x: isize, mut y: isize, dx: isize, dy: isize) -> usize {
    x += dx;
    y += dy;
    while y >= 0 && y < src.len() as isize && x >= 0 && x < src[0].len() as isize {
        match src[y as usize][x as usize] {
            Cell::Occupied => return 1,
            Cell::Empty => return 0,
            Cell::Floor => {}
        }
        x += dx;
        y += dy;
    }
    0
}

fn step(src: &[Vec<Cell>], dst: &mut [Vec<Cell>]) -> usize {
    let (h, w) = (src.len() as isize, src[0].len() as isize);
    let mut changes = 0;
    for y in 0..h {
        for x in 0..w {
            let neighbours = get(src, x, y, -1, -1)
                + get(src, x, y, 0, -1)
                + get(src, x, y, 1, -1)
                + get(src, x, y, 1, 0)
                + get(src, x, y, 1, 1)
                + get(src, x, y, 0, 1)
                + get(src, x, y, -1, 1)
                + get(src, x, y, -1, 0);

            dst[y as usize][x as usize] = match src[y as usize][x as usize] {
                Cell::Empty if neighbours == 0 => {
                    changes += 1;
                    Cell::Occupied
                }
                Cell::Occupied if neighbours >= 5 => {
                    changes += 1;
                    Cell::Empty
                }
                other => other,
            };
        }
    }
    changes
}

fn main() {
    let mut map: Vec<Vec<Cell>> = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();
    let mut map2 = map.clone();

    while step(&map, &mut map2) > 0 {
        mem::swap(&mut map, &mut map2);
    }

    println!(
        "{}",
        map.into_iter()
            .flatten()
            .filter(|&cell| cell == Cell::Occupied)
            .count()
    );
}
