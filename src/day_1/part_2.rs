use std::collections::HashMap;
use std::fs::read_to_string;

pub fn part_2() {
    let input = read_to_string("src/day_1/input.txt").unwrap();
    let split_input: Vec<&str> = input.split("\n").collect();
    let mut first_list: Vec<i32> = vec![];
    let mut second_list: HashMap<i32, i32> = HashMap::new();
    for line in split_input {
        let nums: Vec<&str> = line.split_whitespace().collect();
        first_list.push(nums[0].parse().expect("A number"));
        let count = second_list
            .entry(nums[1].parse::<i32>().expect("A number"))
            .or_insert(0);
        *count += 1;
    }
    let similarity_score = first_list
        .iter()
        .map(|x| x * second_list.get(x).unwrap_or(&0))
        .sum::<i32>();
    println!("{}", similarity_score);
}
