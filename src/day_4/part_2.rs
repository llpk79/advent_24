use std::fs::read_to_string;

fn search(grid: &Vec<Vec<char>>, loc: (usize, usize)) -> i32 {
    let mut lr: String = "".to_string();
    let mut rl: String = "".to_string();
    lr.push(grid[loc.0 + 1][loc.1 - 1]);
    lr.push(grid[loc.0 - 1][loc.1 + 1]);
    rl.push(grid[loc.0 - 1][loc.1 - 1]);
    rl.push(grid[loc.0 + 1][loc.1 + 1]);
    if (lr.contains("SM") || lr.contains("MS")) && (rl.contains("SM") || rl.contains("MS")) {
        1
    } else {
        0
    }
}

pub fn part_2() {
    let input = read_to_string("src/day_4/input.txt").unwrap();
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .into_iter()
        .map(|l| l.trim().chars().collect())
        .collect();
    let mut total = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            if grid[i][j] == 'A' {
                total += search(&grid, (i, j));
            }
        }
    }
    println!("total: {}", total);
}
