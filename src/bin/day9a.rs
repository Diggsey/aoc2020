use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("../../inputs/day9.txt");

fn main() {
    let mut nums = HashMap::<i64, usize>::new();
    let mut queue = VecDeque::new();
    for line in INPUT.lines() {
        let v: i64 = line.parse().unwrap();

        if queue.len() >= 25 {
            if nums.get(&v).copied().unwrap_or_default() == 0 {
                // Not valid
                println!("{}", v);
                break;
            }

            let u = queue.pop_back().unwrap();
            for &item in &queue {
                *nums.get_mut(&(u + item)).unwrap() -= 1;
            }
        }

        for &item in &queue {
            *nums.entry(item + v).or_default() += 1;
        }
        queue.push_front(v);
    }
}
