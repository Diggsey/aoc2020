const INPUT: &str = include_str!("../../inputs/day3.txt");

fn main() {
    let area: Vec<Vec<bool>> = INPUT
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    let width = area[0].len();

    let mut product: i64 = 1;
    for &(dx, dy) in &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let mut x = 0;
        let mut y = 0;
        let mut total = 0;
        while y < area.len() {
            if area[y][x] {
                total += 1;
            }
            x = (x + dx) % width;
            y += dy;
        }
        product *= total;
    }
    println!("{}", product);
}
