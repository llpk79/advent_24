use std::collections::HashMap;
use std::fs::read_to_string;

pub fn part_1() {
    let input = read_to_string("src/day_5/input.txt").unwrap();
    let (str_rules, str_pages) = input.split_once(" ").unwrap();
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in str_rules.split_whitespace() {
        let split = rule.split_once("|").expect("nums");
        let existing = rules
            .entry(split.0.parse::<i32>().unwrap())
            .or_insert(vec![]);
        existing.push(split.1.parse::<i32>().unwrap())
    }
    let pages: Vec<Vec<i32>> = str_pages
        .split_whitespace()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<i32>().expect("num"))
                .collect()
        })
        .collect();
    let mut total: i32 = 0;
    for page in pages {
        let mut ok: bool = true;
        for (i, entry) in page.iter().enumerate() {
            let def = vec![];
            let allowed = rules.get(entry).unwrap_or(&def);
            if i == page.len() - 1 {
                break;
            }
            if page[i + 1..].iter().all(|after| allowed.contains(after)) {
                continue;
            } else {
                ok = false;
                break;
            }
        }
        if ok {
            total += page[page.len() / 2];
        }
    }
    println!("total: {}", total);
}
