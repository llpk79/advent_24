use std::fs::read_to_string;

fn decreasing(report: &[i32]) -> i32 {
    let safe = report
        .windows(2)
        .all(|w| w[1] < w[0] && i32::abs(w[0] - w[1]) <= 3);
    if safe == true {
        1
    } else {
        0
    }
}

fn increasing(report: &[i32]) -> i32 {
    let safe = report
        .windows(2)
        .all(|w| w[0] < w[1] && i32::abs(w[0] - w[1]) <= 3);
    if safe == true {
        1
    } else {
        0
    }
}

pub fn part_2() {
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
    let mut safe_reports: i32 = 0;
    'reports: for report in reports {
        if decreasing(&report) + increasing(&report) == 1 {
            safe_reports += 1;
        } else {
            for i in 0..report.len() {
                let new_report = [&report[..i], &report[i + 1..]].concat();
                if decreasing(&new_report) + increasing(&new_report) == 1 {
                    safe_reports += 1;
                    continue 'reports;
                }
            }
        }
    }
    println!("safe: {:?}", safe_reports);
}

