use std::cmp::max;
use std::fs::read_to_string;

pub fn part_1() {
    let input = read_to_string("src/day_1/input.txt").expect("File exists");
    let split_input: Vec<&str> = input.split("\n").collect();
    let mut first_list: Vec<i32> = vec![];
    let mut second_list: Vec<i32> = vec![];
    for line in split_input {
        let nums: Vec<&str> = line.split_whitespace().collect();
        first_list.push(nums[0].parse().expect("A number"));
        second_list.push(nums[1].parse().expect("A number"));
    }
    first_list.sort();
    second_list.sort();
    let zipped = first_list.iter().zip(second_list.iter());
    let total: i32 = zipped.map(|(a, b)| max(a - b, b - a)).sum();

    println!("{:?}", total);
}
