use regex::Regex;
use std::fs::read_to_string;

pub fn part_2() {
    let input = read_to_string("src/day_3/input.txt").unwrap();
    let muls = Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").unwrap();
    let nums = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut total = 0;
    let mut do_it: bool = true;
    for (toggle,  []) in muls.captures_iter(&input).map(|c| c.extract()) {
        if toggle.contains("do") && !toggle.contains("don't") {
            do_it = true
        } 
        if toggle.contains("don't") {
            do_it = false
        }
        if do_it == true && toggle.contains("mul") {
            for (_, [a, b]) in nums.captures_iter(toggle).map(|c| c.extract()) {
                total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
            }
        }
    }
    println!("total {:?}", total);
}