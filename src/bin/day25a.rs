const INPUT: &str = include_str!("../../inputs/day25.txt");
const BASE: u64 = 7;
const MODULUS: u64 = 20201227;

fn modular_log(value: u64) -> u64 {
    let mut x = 1;
    for res in 0.. {
        if x == value {
            return res;
        }
        x = (x * BASE) % MODULUS;
    }
    unreachable!()
}

fn modular_pow(exponent: u64) -> u64 {
    let n = exponent.leading_zeros();
    let mut res = 1;
    for i in n..64 {
        res = (res * res) % MODULUS;
        if (exponent >> (63 - i)) & 1 == 1 {
            res = (res * BASE) % MODULUS;
        }
    }
    res
}

fn main() {
    let exponent = INPUT
        .lines()
        .map(|line| modular_log(line.parse().unwrap()))
        .product();
    println!("{}", modular_pow(exponent));
}
