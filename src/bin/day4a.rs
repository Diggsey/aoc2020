use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day4.txt");

fn main() {
    let mut num_valid = 0;
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut fields = HashMap::new();
    for line in INPUT.lines() {
        if line.is_empty() {
            if required_fields.iter().all(|f| fields.contains_key(f)) {
                num_valid += 1;
            }
            fields.clear();
        } else {
            for part in line.split(' ') {
                let mut it = part.splitn(2, ':');
                let (k, v) = (it.next().unwrap(), it.next().unwrap());
                fields.insert(k, v);
            }
        }
    }
    println!("{}", num_valid);
}
