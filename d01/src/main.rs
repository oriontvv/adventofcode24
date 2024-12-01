use std::collections::HashMap;

type Number = i32;

fn parse_numbers(s: &str) -> Vec<Number> {
    s.split_whitespace()
        .filter_map(|num| num.trim().parse::<Number>().ok())
        .collect()
}

fn main() {
    let mut vec1 = vec![];
    let mut vec2 = vec![];
    for line in include_str!("input.txt").lines() {
        let nums = parse_numbers(line);
        vec1.push(nums[0]);
        vec2.push(nums[1]);
    }

    vec1.sort();
    vec2.sort();

    // 1
    let sum: Number = vec1
        .iter()
        .zip(vec2.iter())
        .map(|(n1, n2)| (n1 - n2).abs())
        .sum();
    println!("total: {sum}");

    // 2
    let mut counter2 = HashMap::with_capacity(vec2.len());
    for n2 in vec2 {
        *counter2.entry(n2).or_insert(0) += 1;
    }
    let sum: Number = vec1
        .iter()
        .map(|n1| n1 * (*counter2.entry(*n1).or_default()))
        .sum();
    println!("total: {sum}");
}
