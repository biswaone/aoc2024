use std::fs::read_to_string;
use std::collections::HashMap;

fn part1(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut frequency_map = HashMap::new();
    let mut similarity_score = 0;

    for &loc in right.iter() {
        *frequency_map.entry(loc).or_insert(0) += 1;
    }

    for loc in left.iter() {
        match frequency_map.get(loc) {
            Some(frequency) => similarity_score += loc * frequency,
            None => {} 
        }
    }
    similarity_score
}


fn input() -> (Vec<i32>, Vec<i32>) {
    let content = read_to_string("input.txt").unwrap();

    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in content.lines() {
        let locations: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if locations.len() == 2 {
            left.push(locations[0]);
            right.push(locations[1]);
        }
    }

    left.sort();
    right.sort();

    (left, right)
}

fn main() {
    let (left, right) = input();
    let sol1 = part1(&left, &right);
    let sol2 = part2(&left, &right);
    println!("Part A: {:?}", sol1);
    println!("Part B: {:?}", sol2);
}
