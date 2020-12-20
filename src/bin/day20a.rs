use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day20.txt");

struct Tile {
    id: u64,
    data: Vec<Vec<bool>>,
}

fn normalize_edge(edge: u16) -> u16 {
    edge.min(edge.reverse_bits() >> 6)
}

fn to_edge(it: impl Iterator<Item = bool>) -> u16 {
    let mut res = 0;
    for b in it {
        res <<= 1;
        if b {
            res |= 1;
        }
    }
    normalize_edge(res)
}

impl Tile {
    fn top_edge(&self) -> u16 {
        to_edge(self.data.first().unwrap().iter().copied())
    }
    fn bottom_edge(&self) -> u16 {
        to_edge(self.data.last().unwrap().iter().copied())
    }
    fn left_edge(&self) -> u16 {
        to_edge(self.data.iter().map(|v| *v.first().unwrap()))
    }
    fn right_edge(&self) -> u16 {
        to_edge(self.data.iter().map(|v| *v.last().unwrap()))
    }
    fn edges(&self) -> [u16; 4] {
        [
            self.left_edge(),
            self.right_edge(),
            self.top_edge(),
            self.bottom_edge(),
        ]
    }
}

fn main() {
    let tiles: Vec<Tile> = INPUT
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|tile| {
            let mut line_it = tile.lines();
            let tile_id: u64 = line_it
                .next()
                .unwrap()
                .strip_prefix("Tile ")
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse()
                .unwrap();
            Tile {
                id: tile_id,
                data: line_it
                    .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            }
        })
        .collect();

    let mut edge_map: HashMap<u16, Vec<u64>> = HashMap::new();
    for tile in tiles {
        for edge in tile.edges().iter().copied() {
            edge_map.entry(edge).or_default().push(tile.id);
        }
    }

    let mut edge_tiles: HashMap<u64, usize> = HashMap::new();
    for v in edge_map.values() {
        if v.len() == 1 {
            *edge_tiles.entry(v[0]).or_default() += 1;
        }
    }

    let res: u64 = edge_tiles
        .iter()
        .filter(|&(_, v)| *v == 2)
        .map(|(k, _)| *k)
        .product();
    println!("{:?}", res);
}
