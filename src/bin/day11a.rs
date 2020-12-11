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

fn get(src: &[Vec<Cell>], x: isize, y: isize) -> usize {
    if y < 0 || y >= src.len() as isize {
        0
    } else if x < 0 || x >= src[0].len() as isize {
        0
    } else {
        if src[y as usize][x as usize] == Cell::Occupied {
            1
        } else {
            0
        }
    }
}

fn step(src: &[Vec<Cell>], dst: &mut [Vec<Cell>]) -> usize {
    let (h, w) = (src.len() as isize, src[0].len() as isize);
    let mut changes = 0;
    for y in 0..h {
        for x in 0..w {
            let neighbours = get(src, x - 1, y - 1)
                + get(src, x, y - 1)
                + get(src, x + 1, y - 1)
                + get(src, x + 1, y)
                + get(src, x + 1, y + 1)
                + get(src, x, y + 1)
                + get(src, x - 1, y + 1)
                + get(src, x - 1, y);

            dst[y as usize][x as usize] = match src[y as usize][x as usize] {
                Cell::Empty if neighbours == 0 => {
                    changes += 1;
                    Cell::Occupied
                }
                Cell::Occupied if neighbours >= 4 => {
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
