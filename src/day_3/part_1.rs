use regex::Regex;
use std::fs::read_to_string;

pub fn part_1() {
    let input = read_to_string("src/day_3/input.txt").unwrap();
    let muls = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let nums = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut total = 0;
    for (found, _) in muls.captures_iter(&input).map(|c| c.extract::<0>()) {
        for (_, [a, b]) in nums.captures_iter(found).map(|c| c.extract()) {
            total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        }
    }
    println!("mul {:?}", total);
}
