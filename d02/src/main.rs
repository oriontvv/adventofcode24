type Number = i32;

fn parse_numbers(s: &str) -> Vec<Number> {
    s.split_whitespace()
        .filter_map(|num| num.trim().parse::<Number>().ok())
        .collect()
}

fn is_safe(nums: &Vec<Number>) -> bool {
    for (a, b) in nums.iter().zip(nums[1..].iter()) {
        let diff = (a - b).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    nums.is_sorted_by_key(|n| n) || nums.is_sorted_by_key(|n| -n)
}

fn main() {
    // 1
    let safe: Vec<_> = include_str!("input.txt")
        .lines()
        .map(parse_numbers)
        .filter(is_safe)
        .collect();
    println!("safe count p1: {}", safe.len());

    // 2
    let mut count = 0;
    for nums in include_str!("input.txt").lines().map(parse_numbers) {
        if is_safe(&nums) {
            count += 1;
            continue;
        }
        for index in 0..nums.len() {
            let mut nums_1 = nums.clone(); // todo: memory overusage..
            nums_1.remove(index);
            if is_safe(&nums_1) {
                count += 1;
                break;
            }
        }
    }
    println!("safe count p2: {}", count);
}
