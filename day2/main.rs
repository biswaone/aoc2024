use std::fs::read_to_string;

fn input() -> Vec<Vec<i32>> {
    let content = read_to_string("input.txt").unwrap();
    let mut reports: Vec<Vec<i32>> = Vec::new();
    for line in content.lines() {
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        reports.push(report);
    }

    reports
}

fn is_safe(vec: &Vec<i32>) -> bool {
    let seq_ordering = vec[0] > vec[1];
    let mut safe = true;
    for values in vec.windows(2){
        let ordering = values[0] > values[1];
        let valid_diff = (values[0] - values[1]).abs() >= 1 && (values[0] - values[1]).abs() <= 3;
        if !(ordering == seq_ordering) || !valid_diff {
            safe = false;
            break;
        } 
    }
    safe

}

fn part_a(input: &Vec<Vec<i32>>) -> i32 {
    let mut safe: i32 = 0;
    for vec in input {
        if is_safe(&vec){
            safe += 1
        }
    }
    safe
}

fn part_b(input: &Vec<Vec<i32>>) -> i32 {
    let mut safe = 0;
    for vec in input {
        if is_safe(vec) {
            safe += 1;
        } else {
            for i in 0..vec.len() {
                let mut temp_vec = vec.clone();
                temp_vec.remove(i);
                if is_safe(&temp_vec) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    safe
}

fn main() {
    let input = input();
    let sol_a = part_a(&input);
    let sol_b = part_b(&input);
    println!("{:?}", sol_a);
    println!("{:?}", sol_b);
}
