use std::collections::{HashSet, VecDeque};

#[derive(PartialOrd, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Pos {
    row: i32,
    column: i32,
    value: i32,
}

/// Use breadth first search to find a path through a grid of integers that adds to a target value
/// given start and end positions.
/// We start from the finish and subtract for an easier time.
pub fn solve(puzzle: Vec<Vec<i32>>, start: Pos, end: Pos) -> Vec<(i32, i32)> {
    // Setup initial variables.
    let rows = puzzle.len() as i32 - 1;
    let columns = puzzle[0].len() as i32 - 1;
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut paths: VecDeque<Vec<Pos>> = VecDeque::new();
    paths.push_back(vec![end]);

    // Breadth first search.
    while let Some(mut path) = paths.pop_front() {
        // Go up, down, left, and right from last position in the path.
        for (up_down, left_right) in [1, -1, 0, 0].iter().zip([0, 0, 1, -1].iter()) {
            let last_pos = path.last().unwrap();
            let new_row = last_pos.row + *up_down;
            let new_column = last_pos.column + *left_right;

            // Check if the new position is on the puzzle map.
            if 0 <= new_row && new_row <= rows && 0 <= new_column && new_column <= columns {
                // Make a new Pos with value of puzzle position subtracted from running total.
                let new_pos = Pos {
                    row: new_row,
                    column: new_column,
                    value: path.last().expect("valid").value
                        - puzzle[new_row as usize][new_column as usize],
                };
                // If we match our starting position and target value we did it!
                if new_pos == start {
                    // Add the last stop.
                    path.push(new_pos);
                    // Make it the expected direction.
                    path.reverse();
                    // Remove the off puzzle position.
                    path.pop();
                    // Return only the path.
                    return path.iter().map(|pos| (pos.row, pos.column)).collect();
                }
                // If we've been here, or we're below the target value, we don't need to search 
                // this path anymore.
                if visited.contains(&new_pos) || new_pos.value < start.value {
                    continue;
                }
                // Record our visit to this position via this path so we don't loop around.
                visited.insert(new_pos);

                // Make a new path with this position at the end and push it to the back of the queue.
                let mut new_path = path.clone();
                new_path.push(new_pos);
                paths.push_back(new_path);
            }
        }
    }
    vec![(-1, -1)]
}

pub fn solve_it() {
    let puzzle: Vec<Vec<i32>> = vec![vec![12, 8, 13], vec![6, 20, 9], vec![10, 11, 4]];
    let end_value: i32 = 0;
    let start_value: i32 = 50;
    let start = Pos {
        row: 0,
        column: 1,
        value: end_value,
    };
    let end = Pos {
        row: 3,
        column: 1,
        value: start_value,
    };
    println!("{:?}", solve(puzzle, start, end))
}
