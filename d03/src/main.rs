type Num = i64;

fn parse_num(s: &str) -> Num {
    s.trim().parse::<Num>().expect("can't parse number")
}

fn mul(s: &str) -> Num {
    let re = regex::Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
    re.captures_iter(s)
        .map(|m| {
            let (_, x): (&str, [&str; 2]) = m.extract();
            parse_num(x[0]) * parse_num(x[1])
        })
        .sum()
}

fn main() {
    // 1
    let result = mul(include_str!("input.txt"));
    println!("result: {result:?}");

    // 2
    // use some magic with new line in the end of the input text
    let re = regex::Regex::new(r"don't\(\).*?(do\(\)|.\n)").unwrap();
    let s = include_str!("input.txt").replace("\n", "") + "\n";
    let cleaned = re.replace_all(&s, "");
    let result = mul(&cleaned);
    println!("result: {result:?}");
}
