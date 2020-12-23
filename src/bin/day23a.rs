use std::collections::VecDeque;

const INPUT: &str = "792845136";

fn main() {
    let mut values: VecDeque<u32> = INPUT.chars().map(|c| c.to_digit(10).unwrap()).collect();
    for _ in 0..100 {
        let current = values.pop_front().unwrap();
        let moved: Vec<_> = values.drain(0..3).collect();
        let dst_index = values
            .iter()
            .copied()
            .map(|v| current.wrapping_sub(v))
            .enumerate()
            .min_by_key(|x| x.1)
            .unwrap()
            .0;
        for v in moved.into_iter().rev() {
            values.insert(dst_index + 1, v);
        }
        values.push_back(current);
    }
    while let Some(v) = values.pop_front() {
        if v == 1 {
            break;
        } else {
            values.push_back(v);
        }
    }
    let res: String = values
        .into_iter()
        .map(|v| std::char::from_digit(v, 10).unwrap())
        .collect();
    println!("{}", res);
}
