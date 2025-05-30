use std::fs::read_to_string;

pub fn part_1() {
    let input = read_to_string("src/day_9/input.txt").unwrap();
    let numbers: Vec<i32> = input
        .chars()
        .map(|c| c.to_string().parse::<i32>().unwrap())
        .collect();
    let mut right_file_index = numbers.len() - 1;
    let mut right_file_size = numbers[right_file_index];
    let mut right_file_num = numbers.len() / 2;
    let mut left_file_num = 0;
    let mut left_block_pos = 0;
    let mut total = 0;
    'outer: for i in (0..numbers.len() - 1).step_by(2) {
        if left_file_num >= right_file_num && right_file_size > 0 {
            loop {
                println!("t {total}");
                total += right_file_num * left_block_pos;
                left_block_pos += 1;
                right_file_size -= 1;
                if right_file_size == 0 {
                    break 'outer;
                }
            }
        }
        for _ in 0..numbers[i] {
            total += left_file_num * left_block_pos;
            left_block_pos += 1;
        }
        left_file_num += 1;
        'inner: for _ in 0..numbers[i + 1] {
            total += right_file_num * left_block_pos;
            left_block_pos += 1;
            right_file_size -= 1;
            if right_file_size == 0 {
                right_file_index -= 2;
                right_file_size = numbers[right_file_index];
                right_file_num -= 1;
            }
            if left_file_num >= right_file_num {
                break 'inner;
            }
        }
    }
    println!("{}", total);
}

// 10538274524908
// 6203213722586 too high
// 6201390809186 too high
// 6201130364850 wrong
// 6201130364722 *
