use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day7.txt");

struct BagNum {
    num: usize,
    color: &'static str,
}

fn count(
    mapping: &HashMap<&'static str, Vec<BagNum>>,
    cache: &mut HashMap<&'static str, usize>,
    color: &'static str,
) -> usize {
    if let Some(n) = cache.get(&color) {
        *n
    } else {
        let n = if let Some(l) = mapping.get(&color) {
            let mut n = 1;
            for bag_num in l {
                n += count(mapping, cache, bag_num.color) * bag_num.num;
            }
            n
        } else {
            1
        };
        cache.insert(color, n);
        n
    }
}

fn main() {
    let mut mapping: HashMap<_, Vec<_>> = HashMap::new();
    for line in INPUT.lines() {
        let mut it = line.split(" bags contain ");
        let left = it.next().unwrap();
        let right = it.next().unwrap();
        if right != "no other bags." {
            for part in right.split(", ") {
                let mut it2 = part.splitn(2, ' ');
                let num: usize = it2.next().unwrap().parse().unwrap();
                let remainder = it2.next().unwrap();
                let mut it3 = remainder.rsplitn(2, ' ').skip(1);
                let color = it3.next().unwrap();
                mapping.entry(left).or_default().push(BagNum { num, color });
            }
        }
    }
    let mut cache = HashMap::new();
    let res = count(&mapping, &mut cache, "shiny gold") - 1;

    println!("{}", res);
}
