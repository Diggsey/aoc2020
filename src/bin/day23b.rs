use std::mem;

const INPUT: &str = "792845136";

#[derive(Debug)]
struct Cup {
    value: u32,
    circle_next: usize,
    circle_prev: usize,
    num_next: usize,
    num_prev: usize,
}

const TOTAL_LEN: usize = 1000000;

fn main() {
    let early_values: Vec<u32> = INPUT.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let one_pos = early_values.iter().copied().position(|v| v == 1).unwrap();

    let mut values = Vec::new();
    let mut prev_index = TOTAL_LEN - 1;
    for value in early_values.iter().copied() {
        let index = values.len();
        values.push(Cup {
            value,
            circle_prev: prev_index,
            circle_next: index + 1,
            num_prev: early_values
                .iter()
                .copied()
                .position(|v| v == value - 1)
                .unwrap_or(TOTAL_LEN - 1),
            num_next: early_values
                .iter()
                .copied()
                .position(|v| v == value + 1)
                .unwrap_or(early_values.len()),
        });
        prev_index = index;
    }
    for value in 10..=TOTAL_LEN {
        values.push(Cup {
            value: value as u32,
            circle_prev: value - 2,
            circle_next: value % TOTAL_LEN,
            num_prev: if value == 10 {
                early_values.iter().copied().position(|v| v == 9).unwrap()
            } else {
                value - 2
            },
            num_next: if value == TOTAL_LEN { one_pos } else { value },
        })
    }

    let mut current = 0;
    for _ in 0..10000000 {
        let move1 = values[current].circle_next;
        let move2 = values[move1].circle_next;
        let move3 = values[move2].circle_next;
        let move_end = values[move3].circle_next;

        let mut dst = values[current].num_prev;
        while dst == move1 || dst == move2 || dst == move3 {
            dst = values[dst].num_prev;
        }

        values[move_end].circle_prev = current;
        values[current].circle_next = move_end;

        let dst_end = mem::replace(&mut values[dst].circle_next, move1);
        values[dst_end].circle_prev = move3;
        values[move1].circle_prev = dst;
        values[move3].circle_next = dst_end;

        current = move_end;
    }
    let a = values[one_pos].circle_next;
    let b = values[a].circle_next;
    let res = (values[a].value as u64) * (values[b].value as u64);
    println!("{}", res);
}
