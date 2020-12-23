use std::mem;

const INPUT: &str = "792845136";
const TOTAL_LEN: usize = 1000000;

fn prev(n: usize) -> usize {
    (n + TOTAL_LEN - 1) % TOTAL_LEN
}

fn main() {
    let mut values: Vec<_> = (0..TOTAL_LEN).map(|v| v + 1).collect();
    let mut current = 9;
    for c in INPUT.chars().rev() {
        let v = (c.to_digit(10).unwrap() - 1) as usize;
        values[v] = current;
        current = v;
    }
    values[TOTAL_LEN - 1] = current;
    for _ in 0..10000000 {
        let move1 = values[current];
        let move2 = values[move1];
        let move3 = values[move2];
        let move_end = values[move3];

        let mut dst = prev(current);
        while dst == move1 || dst == move2 || dst == move3 {
            dst = prev(dst);
        }

        values[current] = move_end;

        let dst_end = mem::replace(&mut values[dst], move1);
        values[move3] = dst_end;
        current = move_end;
    }
    let a = values[0];
    let b = values[a];
    let res = ((a + 1) as u64) * ((b + 1) as u64);
    println!("{}", res);
}
