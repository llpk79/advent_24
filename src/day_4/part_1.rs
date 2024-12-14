use std::fs::read_to_string;

fn up(grid: &Vec<Vec<char>>, loc: (usize, usize)) -> i32 {
    if loc.0 < 3 {
        return 0;
    }
    let mut xmas: String = "".to_string();
    for i in 0..4 {
        xmas.push(grid[loc.0 - i][loc.1]);
    }
    if xmas.contains("XMAS") {
        1
    } else {
        0
    }
}

fn down(grid: &Vec<Vec<char>>, loc: (usize, usize)) -> i32 {
    if loc.0 >= grid.len() - 3 {
        return 0;
    }
    let mut xmas: String = "".to_string();
    for i in 0..4 {
        xmas.push(grid[loc.0 + i][loc.1]);
    }
    if xmas.contains("XMAS") {
        1
    } else {
        0
    }
}

fn left(grid: &Vec<Vec<char>>, loc: (usize, usize)) -> i32 {
    if loc.1 < 3 {
        return 0;
    }
    let mut xmas: String = "".to_string();
    for i in 0..4 {
        xmas.push(grid[loc.0][loc.1 - i]);
    }
    if xmas.contains("XMAS") {
        1
    } else {
        0
    }
}

fn right(grid: &Vec<Vec<char>>, loc: (usize, usize)) -> i32 {
    if loc.1 >= grid[0].len() - 3 {
        return 0;
    }
    let mut xmas: String = "".to_string();
    for i in 0..4 {
        xmas.push(grid[loc.0][loc.1 + i]);
    }
    if xmas.contains("XMAS") {
        1
    } else {
        0
    }
}

fn down_left(grid: &Vec<Vec<char>>, loc: (usize, usize)) -> i32 {
    if loc.0 >= grid.len() - 3 || loc.1 < 3 {
        return 0;
    }
    let mut xmas: String = "".to_string();
    for i in 0..4 {
        xmas.push(grid[loc.0 + i][loc.1 - i]);
    }
    if xmas.contains("XMAS") {
        1
    } else {
        0
    }
}

fn down_right(grid: &Vec<Vec<char>>, loc: (usize, usize)) -> i32 {
    if loc.0 >= grid.len() - 3 || loc.1 >= grid[0].len() - 3 {
        return 0;
    }
    let mut xmas: String = "".to_string();
    for i in 0..4 {
        xmas.push(grid[loc.0 + i][loc.1 + i]);
    }
    if xmas.contains("XMAS") {
        1
    } else {
        0
    }
}

fn up_left(grid: &Vec<Vec<char>>, loc: (usize, usize)) -> i32 {
    if loc.0 < 3 || loc.1 < 3 {
        return 0;
    }
    let mut xmas: String = "".to_string();
    for i in 0..4 {
        xmas.push(grid[loc.0 - i][loc.1 - i]);
    }
    if xmas.contains("XMAS") {
        1
    } else {
        0
    }
}

fn up_right(grid: &Vec<Vec<char>>, loc: (usize, usize)) -> i32 {
    if loc.0 < 3 || loc.1 >= grid[0].len() - 3 {
        return 0;
    }
    let mut xmas: String = "".to_string();
    for i in 0..4 {
        xmas.push(grid[loc.0 - i][loc.1 + i]);
    }
    if xmas.contains("XMAS") {
        1
    } else {
        0
    }
}

pub fn part_1() {
    let input = read_to_string("src/day_4/input.txt").unwrap();
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .into_iter()
        .map(|l| l.trim().chars().collect())
        .collect();
    let mut total = 0;
    for i in 0..grid.clone().len() {
        for j in 0..grid.clone()[0].len() {
            total += (up(&grid, (i, j))
                + down(&grid, (i, j))
                + left(&grid, (i, j))
                + right(&grid, (i, j))
                + up_right(&grid, (i, j))
                + up_left(&grid, (i, j))
                + down_left(&grid, (i, j))
                + down_right(&grid, (i, j)));
        }
    }
    println!("{}", total);
}
