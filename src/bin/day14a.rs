use std::collections::HashMap;

use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/day14.txt");

#[derive(Copy, Clone, Debug)]
enum Instr {
    SetMask { mask: u64, pattern: u64 },
    SetMemory { address: u64, value: u64 },
}

fn main() {
    let program: Vec<Instr> = INPUT
        .lines()
        .map(|line| {
            if let Ok(mask_str) = scan_fmt!(line, "mask = {}", String) {
                let mut mask = 0;
                let mut pattern = 0;
                for c in mask_str.chars() {
                    mask <<= 1;
                    pattern <<= 1;
                    match c {
                        'X' => mask |= 1,
                        '1' => pattern |= 1,
                        '0' => {}
                        _ => unreachable!(),
                    }
                }
                Instr::SetMask { mask, pattern }
            } else if let Ok((address, value)) = scan_fmt!(line, "mem[{}] = {}", u64, u64) {
                Instr::SetMemory { address, value }
            } else {
                unreachable!()
            }
        })
        .collect();
    let mut memory = HashMap::new();
    let mut cur_mask = 0;
    let mut cur_pattern = 0;
    for instr in program {
        match instr {
            Instr::SetMask { mask, pattern } => {
                cur_mask = mask;
                cur_pattern = pattern;
            }
            Instr::SetMemory { address, value } => {
                *memory.entry(address).or_default() = (value & cur_mask) | cur_pattern;
            }
        }
    }
    let total: u64 = memory.values().sum();
    println!("{}", total);
}
