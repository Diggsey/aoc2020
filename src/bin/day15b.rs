use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day15.txt");

#[derive(Default)]
struct State {
    last_seen: HashMap<u64, u64>,
    index: u64,
    prev_index: Option<u64>,
    prev: u64,
}

impl State {
    fn visit(&mut self, value: u64) {
        self.prev = value;
        self.prev_index = self.last_seen.insert(value, self.index);
        self.index += 1;
    }
    fn step(&mut self) {
        if let Some(n) = self.prev_index {
            self.visit(self.index - n - 1);
        } else {
            self.visit(0);
        }
    }
}

fn main() {
    let mut state = State::default();
    for value in INPUT.split(',') {
        state.visit(value.parse().unwrap());
    }
    while state.index < 30000000 {
        state.step();
    }
    println!("{}", state.prev);
}
