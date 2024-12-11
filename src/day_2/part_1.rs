use std::fs::read_to_string;

fn decreasing(report: Vec<i32>) -> i32 {
    let safe = report
        .windows(2)
        .all(|w| w[1] < w[0] && i32::abs(w[0] - w[1]) <= 3);
    if safe == true {
        1
    } else {
        0
    }
}

fn increasing(report: Vec<i32>) -> i32 {
    let safe = report
        .windows(2)
        .all(|w| w[0] < w[1] && i32::abs(w[0] - w[1]) <= 3);
    if safe == true {
        1
    } else {
        0
    }
}

pub fn part_1() {
    let input = read_to_string("src/day_2/input.txt").unwrap();
    let str_lines: Vec<&str> = input.split("\n").collect();
    let split_str_lines: Vec<Vec<&str>> = str_lines
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let reports: Vec<Vec<i32>> = split_str_lines
        .iter()
        .map(|line| {
            line.iter()
                .map(|c| c.parse::<i32>().expect("num"))
                .collect()
        })
        .collect();
    let safe_reports: i32 = reports
        .iter()
        .map(|report| decreasing(report.clone()) + increasing(report.clone()))
        .sum();
    println!("safe: {:?}", safe_reports);
}
