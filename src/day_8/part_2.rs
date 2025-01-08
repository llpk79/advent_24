use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn part_2() {
    let input = read_to_string("src/day_8/input.txt").unwrap();
    let mut the_map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    let len: isize = input.lines().count() as isize;
    for (i, line) in input.split_whitespace().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char != '.' {
                the_map
                    .entry(char)
                    .or_default()
                    .push((i as isize, j as isize));
            }
        }
    }
    let mut antinodes: Vec<(isize, isize)> = Vec::new();
    for (_, points) in &the_map {
        for (i, point) in points.iter().enumerate() {
            if i == points.len() - 1 {
                break;
            }
            for other_point in points[i + 1..].iter() {
                antinodes.push(*point);
                antinodes.push(*other_point);
                let y_adj = (point.0 - other_point.0).abs();
                let x_adj = (point.1 - other_point.1).abs();
                let mut x_act = x_adj;
                let mut y_act = y_adj;
                'inner: loop {
                    if point.1 < other_point.1 {
                        antinodes.push((point.0 - y_act, point.1 - x_act));
                        antinodes.push((other_point.0 + y_act, other_point.1 + x_act));
                    } else {
                        antinodes.push((point.0 - y_act, point.1 + x_act));
                        antinodes.push((other_point.0 + y_act, other_point.1 - x_act));
                    }
                    y_act += y_adj;
                    x_act += x_adj;
                    println!("y {y_act}\nx {x_act}");
                    if (y_act > len && x_act > len) || (x_act < 0 && y_act < 0) {
                        break 'inner;
                    }
                }
            }
        }
    }
    let total: HashSet<&(isize, isize)> = antinodes
        .iter()
        .filter(|x| x.0 < len && x.1 < len && x.0 >= 0 && x.1 >= 0)
        .collect();
    println!("{:?}\n{}", total, total.iter().count());
}
