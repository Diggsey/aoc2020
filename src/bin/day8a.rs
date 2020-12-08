use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &str = include_str!("../../inputs/day8.txt");

#[derive(Debug, Copy, Clone)]
enum Op {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            "nop" => Self::Nop,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Instr {
    op: Op,
    arg: i64,
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 5 {
            return Err(());
        }
        let op = s[0..3].parse()?;
        let arg = s[4..].parse().map_err(|_| ())?;
        Ok(Self { op, arg })
    }
}

#[derive(Debug, Clone)]
struct State {
    program: Vec<Instr>,
    acc: i64,
    ip: i64,
}

impl State {
    fn new(program: Vec<Instr>) -> Self {
        Self {
            program,
            acc: 0,
            ip: 0,
        }
    }
    fn advance(&mut self) {
        let instr = self.program[self.ip as usize];
        match instr.op {
            Op::Nop => {}
            Op::Acc => self.acc += instr.arg,
            Op::Jmp => {
                self.ip += instr.arg;
                return;
            }
        }
        self.ip += 1;
    }
}

fn main() {
    let mut state = State::new(INPUT.lines().map(|line| line.parse().unwrap()).collect());
    let mut visited_ips = HashSet::new();

    while visited_ips.insert(state.ip) {
        state.advance();
    }
    println!("{}", state.acc);
}
