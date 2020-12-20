use std::collections::HashMap;
use std::ops::{Add, Neg, Sub};

const INPUT: &str = include_str!("../../inputs/day20.txt");

struct Tile {
    data: Vec<Vec<bool>>,
}

// Operation necessary to get the target edge to be on the left
// of the tile, from top to bottom.
#[derive(Copy, Clone, Debug)]
struct Orientation {
    rotate: u8,
    flip: bool,
}

impl Orientation {
    fn apply(&self, mut offset: (i32, i32)) -> (i32, i32) {
        for _ in 0..self.rotate {
            offset = (-offset.1, offset.0);
        }
        if self.flip {
            offset.1 = -offset.1;
        }
        offset
    }
    fn apply_tile(&self, mut offset: (usize, usize)) -> (usize, usize) {
        for _ in 0..self.rotate {
            offset = (9 - offset.1, offset.0);
        }
        if self.flip {
            offset.1 = 9 - offset.1;
        }
        offset
    }
}

impl Add for Orientation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.flip {
            Self {
                rotate: (self.rotate + 4 - rhs.rotate) % 4,
                flip: !rhs.flip,
            }
        } else {
            Self {
                rotate: (self.rotate + rhs.rotate) % 4,
                flip: rhs.flip,
            }
        }
    }
}

impl Neg for Orientation {
    type Output = Self;

    fn neg(self) -> Self {
        if self.flip {
            Self {
                rotate: self.rotate,
                flip: true,
            }
        } else {
            Self {
                rotate: (4 - self.rotate) % 4,
                flip: false,
            }
        }
    }
}

impl Sub for Orientation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

#[derive(Copy, Clone, Debug)]
struct Edge {
    pat: u16,
    orientation: Orientation,
}

#[derive(Copy, Clone, Debug)]
struct OrientedTile {
    index: usize,
    orientation: Orientation,
}

fn normalize_edge(mut edge: Edge) -> Edge {
    let rev_pat = edge.pat.reverse_bits() >> 6;
    if rev_pat < edge.pat {
        edge.orientation.flip = !edge.orientation.flip;
        edge.pat = rev_pat;
    }
    edge
}

fn to_edge(it: impl Iterator<Item = bool>, orientation: Orientation) -> Edge {
    let mut pat = 0;
    for b in it {
        pat <<= 1;
        if b {
            pat |= 1;
        }
    }
    normalize_edge(Edge { pat, orientation })
}

impl Tile {
    fn top_edge(&self) -> Edge {
        to_edge(
            self.data.first().unwrap().iter().copied(),
            Orientation {
                rotate: 3,
                flip: true,
            },
        )
    }
    fn bottom_edge(&self) -> Edge {
        to_edge(
            self.data.last().unwrap().iter().copied(),
            Orientation {
                rotate: 1,
                flip: false,
            },
        )
    }
    fn left_edge(&self) -> Edge {
        to_edge(
            self.data.iter().map(|v| *v.first().unwrap()),
            Orientation {
                rotate: 0,
                flip: false,
            },
        )
    }
    fn right_edge(&self) -> Edge {
        to_edge(
            self.data.iter().map(|v| *v.last().unwrap()),
            Orientation {
                rotate: 2,
                flip: true,
            },
        )
    }
    fn edges(&self) -> [Edge; 4] {
        [
            self.left_edge(),
            self.right_edge(),
            self.top_edge(),
            self.bottom_edge(),
        ]
    }
}

fn find_pattern(map: &Vec<Vec<bool>>, pat: &Vec<(usize, usize)>) -> usize {
    let pat_w = pat.iter().map(|p| p.0).max().unwrap() + 1;
    let pat_h = pat.iter().map(|p| p.1).max().unwrap() + 1;
    let mut count = 0;
    for y in 0..map.len() - pat_h {
        'next: for x in 0..map[0].len() - pat_w {
            for (dx, dy) in pat {
                if !map[y + dy][x + dx] {
                    continue 'next;
                }
            }
            count += 1;
        }
    }
    count
}

fn main() {
    let tiles: Vec<Tile> = INPUT
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|tile| {
            let mut line_it = tile.lines();
            let _tile_id: u64 = line_it
                .next()
                .unwrap()
                .strip_prefix("Tile ")
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse()
                .unwrap();
            Tile {
                data: line_it
                    .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            }
        })
        .collect();

    let mut edge_map: HashMap<u16, Vec<OrientedTile>> = HashMap::new();
    for (index, tile) in tiles.iter().enumerate() {
        for edge in tile.edges().iter().copied() {
            edge_map.entry(edge.pat).or_default().push(OrientedTile {
                index,
                orientation: edge.orientation,
            });
        }
    }

    let mut positioned_tiles = HashMap::new();
    positioned_tiles.insert(
        (0, 0),
        OrientedTile {
            index: 0,
            orientation: Orientation {
                rotate: 2,
                flip: true,
            },
        },
    );
    let mut tile_queue = vec![(0, 0)];
    while let Some(pos) = tile_queue.pop() {
        let positioned_tile = positioned_tiles[&pos];
        for edge in tiles[positioned_tile.index].edges().iter().copied() {
            let orientation = -edge.orientation + positioned_tile.orientation;
            let offset = orientation.apply((-1, 0));
            let new_pos = (pos.0 + offset.0, pos.1 + offset.1);
            if positioned_tiles.contains_key(&new_pos) {
                continue;
            }
            let options: Vec<_> = edge_map[&edge.pat]
                .iter()
                .copied()
                .filter(|tile| tile.index != positioned_tile.index)
                .collect();

            if options.len() == 1 {
                let new_tile = options[0];
                let new_orientation = new_tile.orientation
                    + Orientation {
                        rotate: 2,
                        flip: true,
                    }
                    + orientation;
                positioned_tiles.insert(
                    new_pos,
                    OrientedTile {
                        index: new_tile.index,
                        orientation: new_orientation,
                    },
                );
                tile_queue.push(new_pos);
            }
        }
    }

    let min_x = positioned_tiles.keys().map(|k| k.0).min().unwrap();
    let min_y = positioned_tiles.keys().map(|k| k.1).min().unwrap();
    let max_x = positioned_tiles.keys().map(|k| k.0).max().unwrap();
    let max_y = positioned_tiles.keys().map(|k| k.1).max().unwrap();

    let positioned_tiles = &positioned_tiles;
    let tiles = &tiles;
    let full_map = (min_y..=max_y)
        .flat_map(|y| {
            (1..9).map(move |dy| {
                (min_x..=max_x)
                    .flat_map(|x| {
                        let tile = positioned_tiles[&(x, y)];
                        let orientation = -tile.orientation;
                        (1..9).map(move |dx| {
                            let pos = orientation.apply_tile((dx, dy));
                            tiles[tile.index].data[pos.1][pos.0]
                        })
                    })
                    .collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();

    let sea_monster_str = "                  # \n#    ##    ##    ###\n #  #  #  #  #  #   \n";
    let sea_monster_pat: Vec<_> = sea_monster_str
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    let count = find_pattern(&full_map, &sea_monster_pat);
    let total = full_map.iter().flatten().filter(|&&b| b).count();
    let res = total - count * sea_monster_pat.len();
    println!("{}", res);
}
