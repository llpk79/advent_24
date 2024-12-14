use std::collections::HashMap;
use std::fs::read_to_string;

fn part_1() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
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
    let mut incorrect: Vec<Vec<i32>> = vec![];
    for page in pages {
        for (i, entry) in page.iter().enumerate() {
            let def = vec![];
            let allowed = rules.get(entry).unwrap_or(&def);
            if i == page.len() - 1 {
                break;
            }
            if page[i + 1..].iter().all(|after| allowed.contains(after)) {
                continue;
            } else {
                incorrect.push(page);
                break;
            }
        }
    }
    (rules, incorrect)
}

fn fix(rules: &HashMap<i32, Vec<i32>>, page: &Vec<i32>) -> Option<i32> {
    let def = vec![];
    if page.iter().enumerate().all(|(i, x)| {
        page[i + 1..]
            .iter()
            .all(|after| rules.get(x).unwrap_or(&def).contains(after))
    }) {
        return Some(page[page.len() / 2]);
    }
    'entries: for (i, entry) in page.iter().enumerate() {
        let def = vec![];
        let allowed = rules.get(entry).unwrap_or(&def);
        if i == page.len() - 1 {
            return Some(page[page.len() / 2]);
        }
        if page[i + 1..].iter().all(|after| allowed.contains(after)) {
            continue 'entries;
        } else {
            let mut offender: usize = 0;
            'inner: for j in i + 1..page.len() {
                if !allowed.contains(&page[j]) {
                    offender = j;
                    break 'inner;
                }
            }
            let mut start: Vec<i32> = [page[offender]].to_vec();
            start.push(*entry);
            start = [page[..i].to_vec(), start].concat();
            let mid: Vec<i32> = page[i + 1..offender].to_vec();
            let finish: Vec<i32> = page[offender + 1..].to_vec();
            let new_page: Vec<i32> = [start, mid, finish].concat();
            assert_eq!(page.len(), new_page.len());
            return fix(rules, &new_page);
        }
    }
    Some(0)
}

pub fn part_2() {
    let (rules, incorrect_pages) = part_1();
    let total: i32 = incorrect_pages
        .iter()
        .map(|page| fix(&rules, page).expect("valid"))
        .sum();
    println!("total: {}", total);
}
