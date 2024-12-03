use regex::Regex;
use std::fs::read_to_string;

fn input() -> String{
    let content = read_to_string("input.txt").unwrap();
    content
}

fn part_a(corrupted_memory: &String) -> i32{
    let mut total_sum = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in re.captures_iter(&corrupted_memory) {
        if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
            let x_str = x.as_str();
            let y_str = y.as_str();

            if let (Ok(x_num), Ok(y_num)) = (x_str.parse::<i32>(), y_str.parse::<i32>()) {
                total_sum += x_num * y_num;
            }
    
        }
    }
    total_sum
    
}

fn main() {
    let corrupted_memory = input();
    let sol_a = part_a(&corrupted_memory);
    println!("{}",sol_a);
    
}

