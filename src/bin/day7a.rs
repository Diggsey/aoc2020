use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day7.txt");

fn main() {
    let mut mapping: HashMap<_, Vec<_>> = HashMap::new();
    for line in INPUT.lines() {
        let mut it = line.split(" bags contain ");
        let left = it.next().unwrap();
        let right = it.next().unwrap();
        if right != "no other bags." {
            for part in right.split(", ") {
                let mut it2 = part.splitn(2, ' ');
                let _num: usize = it2.next().unwrap().parse().unwrap();
                let remainder = it2.next().unwrap();
                let mut it3 = remainder.rsplitn(2, ' ').skip(1);
                let color = it3.next().unwrap();
                mapping.entry(color).or_default().push(left);
            }
        }
    }
    let mut visited = HashSet::new();
    let mut to_visit = Vec::new();
    to_visit.push("shiny gold");
    while let Some(color) = to_visit.pop() {
        if visited.insert(color) {
            if let Some(l) = mapping.get(&color) {
                to_visit.extend(l.iter().copied());
            }
        }
    }

    println!("{}", visited.len() - 1);
}
