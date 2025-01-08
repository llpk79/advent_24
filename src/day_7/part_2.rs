use std::fs::read_to_string;

struct Equation {
    value: i64,
    nums: Vec<i64>,
}

impl Equation {
    fn solve(&self) -> i64 {
        let mut prev: Vec<i64> = Vec::new();
        for (i, window) in self.nums.windows(2).enumerate() {
            if prev.is_empty() {
                let concat: i64 = (window[0].to_string() + &window[1].to_string())
                    .parse()
                    .unwrap();
                prev.push(concat);
                prev.push(window[0] + window[1]);
                prev.push(window[0] * window[1]);
                continue;
            } else {
                for pre in &prev.clone()[(i - 1) * 3..] {
                    let concat: i64 = (pre.to_string() + &window[1].to_string()).parse().unwrap();
                    prev.push(concat);
                    prev.push(*pre + window[1]);
                    prev.push(*pre * window[1]);
                }
            }
        }
        println!(
            "\n {}\n{:?}",
            self.value,
            &prev[(self.nums.len() - 2) * 3..]
        );
        if prev[(self.nums.len() - 2) * 3..].contains(&self.value) {
            return self.value;
        }
        0
    }
}

pub fn part_2() {
    let input = read_to_string("src/day_7/example.txt").unwrap();
    let equations: i64 = input
        .split("\n")
        .map(|line| {
            let (value, nums) = line.split_once(": ").expect("std");
            Equation {
                value: value.parse().unwrap(),
                nums: nums
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            }
            .solve()
        })
        .sum();
    println!("{}", equations);
}

// 227655036931221
// 227655020013362
