use std::fs;

pub fn main() {
    let input = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let mut total_increases = 0;
    let mut slow_cursor = 0;
    let mut fast_cursor = 1;
    
    while fast_cursor < numbers.len() - 2 {
        let current_window = numbers[slow_cursor] + numbers[slow_cursor + 1] + numbers[slow_cursor + 2];
        let next_window = numbers[fast_cursor] + numbers[fast_cursor + 1] + numbers[fast_cursor + 2];
        
        if next_window > current_window {
            total_increases += 1;
        }

        fast_cursor += 1;
        slow_cursor += 1;
    }

    println!("{}", total_increases)
}
