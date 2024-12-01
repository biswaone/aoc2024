use std::collections::HashMap;


fn solve() {
    let contents = std::fs::read_to_string("input.txt").expect("Failed to read file");
    let mut locations1 = Vec::new();
    let mut locations2 = Vec::new();
    let mut frequency_map = HashMap::new();

    for line in contents.lines() {
        let locations: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        locations1.push(locations[0]);
        locations2.push(locations[1]);
    }
    locations1.sort();
    locations2.sort();
    let sum: i32 = locations1.iter()
    .zip(locations2.iter())
    .map(|(a, b)| (a - b).abs())
    .sum();
    println!("{:?}", sum);

    for &loc in &locations2 {
        *frequency_map.entry(loc).or_insert(0) += 1;
    }

   let mut similarity_score = 0;
    for loc in locations1 {
        match frequency_map.get(&loc) {
            Some(frequency) => similarity_score += loc * frequency,
            None => similarity_score += 0,
        }
    }
   println!("{:?}", similarity_score);

}

fn main() {
    solve()
}
