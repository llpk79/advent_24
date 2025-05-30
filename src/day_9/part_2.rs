use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::read_to_string;

pub fn part_2() {
    let input = read_to_string("src/day_9/example.txt").unwrap();
    let numbers: Vec<i32> = input
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect();
    let rev_numbers: Vec<i32> = numbers.iter().rev().copied().collect();
    let len = rev_numbers.len();
    let mut gap_map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for i in (1..len).step_by(2) {
        if let Some(vec) = gap_map.get_mut(&rev_numbers[i]) {
            vec.push((((len - i) / 2) - 1) as i32);
        } else {
            gap_map.insert(rev_numbers[i], vec![(((len - i) / 2) - 1) as i32]);
        }
    }
    println!("{:?}", gap_map);

    for i in (1..len).step_by(2) {
        println!("num {}", numbers[i]);
        if let Some((_, vec)) = gap_map.iter().find(|(k, _)| **k >= numbers[i]) {
            println!("{:?}", vec);
        }
    }
}
