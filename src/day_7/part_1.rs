use std::collections::HashSet;
use std::fs::read_to_string;

struct Equation {
    value: i64,
    nums: Vec<i64>,
}

impl Equation {
    fn solve(&self) -> i64 {
        let mut prev: HashSet<i64> = HashSet::new();
        for window in self.nums.windows(2) {
            if prev.is_empty() {
                prev.insert(window[0] + window[1]);
                prev.insert(window[0] * window[1]);
                continue;
            }
            for pre in prev.clone().iter() {
                prev.insert(*pre + window[1]);
                prev.insert(*pre * window[1]);
            }
        }
        if prev.contains(&self.value) {
            return self.value;
        }
        0
    }
}

pub fn part_1() {
    let input = read_to_string("src/day_7/input.txt").unwrap();
    let equations: i64 = input
        .split("\n")
        .map(|line| {
            // println!("line: {}", line);
            let (value, nums) = line.split_once(": ").expect("std");
            Equation {
                value: value.parse::<i64>().unwrap(),
                nums: nums
                    .split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect(),
            }
            .solve()
        })
        .sum();
    println!("{}", equations);
}
