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

fn part_b(corrupted_memory: &String) -> i32{
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut flag = true; 
    let mut total_sum = 0;

    for mat in re.find_iter(&corrupted_memory) {
        let match_str = mat.as_str();
        if match_str == "do()" {
            flag = true;
        } else if match_str == "don't()" {
            flag = false;
        } else if flag && match_str.starts_with("mul(") {
            let parts: Vec<&str> = match_str[4..match_str.len() - 1].split(',').collect();
            if let (Ok(x), Ok(y)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                total_sum += x * y;
            }
        }
    }

    total_sum
}

fn main() {
    let corrupted_memory = input();
    let sol_a = part_a(&corrupted_memory);
    let sol_b = part_b(&corrupted_memory);
    println!("{}",sol_a);
    println!("{}",sol_b);
    
}

