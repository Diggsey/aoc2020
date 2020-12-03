const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    let area: Vec<Vec<bool>> = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let width = area[0].len();

    let mut x = 0;
    let mut total = 0;
    for row in area.iter() {
        if row[x] {
            total += 1;
        }
        x = (x + 3) % width;
    }
    println!("{}", total);
}
