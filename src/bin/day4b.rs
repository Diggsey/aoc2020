use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day4.txt");

fn validate_num(value: &str, min: i32, max: i32) -> bool {
    value
        .parse()
        .map(|v: i32| v >= min && v <= max)
        .unwrap_or_default()
}

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
                let valid = match k {
                    "byr" => validate_num(v, 1920, 2002),
                    "iyr" => validate_num(v, 2010, 2020),
                    "eyr" => validate_num(v, 2020, 2030),
                    "hgt" => {
                        if v.ends_with("cm") {
                            validate_num(&v[0..v.len() - 2], 150, 193)
                        } else if v.ends_with("in") {
                            validate_num(&v[0..v.len() - 2], 59, 76)
                        } else {
                            false
                        }
                    }
                    "hcl" => {
                        v.len() == 7
                            && v.starts_with('#')
                            && v[1..]
                                .chars()
                                .all(|c| (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f'))
                    }
                    "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
                    "pid" => v.len() == 9 && v.chars().all(|c| c >= '0' && c <= '9'),
                    _ => true,
                };
                if valid {
                    fields.insert(k, v);
                }
            }
        }
    }
    println!("{}", num_valid);
}
