use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct Directions {
    up: (isize, isize),
    down: (isize, isize),
    left: (isize, isize),
    right: (isize, isize),
}

impl Directions {
    pub fn new() -> Directions {
        Directions {
            up: (-1, 0),
            down: (1, 0),
            left: (0, -1),
            right: (0, 1),
        }
    }
}

pub fn part_2() {
    let input = read_to_string("src/day_6/input.txt").unwrap();
    let mut grid: Vec<Vec<char>> = input
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
    let direction = Directions::new();
    let directions: Vec<(isize, isize)> = vec![
        direction.up,
        direction.right,
        direction.down,
        direction.left,
    ];
    let mut total: i32 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if (i as isize, j as isize) == initial_pos {
                continue;
            }
            let original_char = grid[i][j];
            if original_char != '#' {
                grid[i][j] = '#';
                total += is_loop(&directions, initial_pos, &grid);
                grid[i][j] = original_char;
            }
        }
    }
    println!("total: {}", total);
}

fn is_loop(
    directions: &Vec<(isize, isize)>,
    mut initial_pos: (isize, isize),
    grid: &Vec<Vec<char>>,
) -> i32 {
    let mut turns: HashSet<(isize, isize)> = HashSet::new();
    let mut pre_turns: HashSet<(isize, isize)> = HashSet::new();
    loop {
        for direction in directions.iter() {
            pre_turns = turns.clone();
            let mut step: (isize, isize);
            'inner: loop {
                step = (initial_pos.0 + direction.0, initial_pos.1 + direction.1);
                if step.0 < 0
                    || step.0 > (grid.len() - 1) as isize
                    || step.1 < 0
                    || step.1 > (grid[0].len() - 1) as isize
                {
                    return 0;
                }
                if grid[step.0 as usize][step.1 as usize] != '#' {
                    initial_pos = step;
                } else {
                    break 'inner;
                }
            }
        }
        turns.insert((initial_pos.0, initial_pos.1));
        if turns.iter().all(|turn| pre_turns.contains(turn)) {
            return 1;
        }
    }
}
