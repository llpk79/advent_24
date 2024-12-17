use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct directions {
    up: (isize, isize),
    down: (isize, isize),
    left: (isize, isize),
    right: (isize, isize),
}

impl directions {
    pub fn new() -> directions {
        directions {
            up: (-1, 0),
            down: (1, 0),
            left: (0, -1),
            right: (0, 1),
        }
    }
}

pub fn part_1() {
    let input = read_to_string("src/day_6/input.txt").unwrap();
    let grid: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|line| line.chars().collect())
        .collect();
    let mut initial_pos: (isize, isize) = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                initial_pos = (i as isize, j as isize);
            }
        }
    }
    let direction = directions::new();
    let directions: Vec<(isize, isize)> = vec![
        direction.up,
        direction.right,
        direction.down,
        direction.left,
    ];
    let mut step_count: HashSet<(isize, isize)> = HashSet::new();
    'outer: loop {
        for direction in directions.iter() {
            'inner: loop {
                let step = (initial_pos.0 + direction.0, initial_pos.1 + direction.1);
                if grid[step.0 as usize][step.1 as usize] != '#' {
                    if step.0 < 0
                        || step.0 >= (grid.len() - 1) as isize
                        || step.1 < 0
                        || step.1 >= (grid[0].len() - 1) as isize
                    {
                        break 'outer;
                    }
                    initial_pos = step;
                    step_count.insert((step.0, step.1));
                } else {
                    break 'inner;
                }
            }
        }
    }
    println!("{:?}", step_count.len() + 1);
}
