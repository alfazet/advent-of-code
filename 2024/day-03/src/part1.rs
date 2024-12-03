use regex::Regex;

pub fn solve(input: String) {
    let regex = Regex::new(r#"mul\(([0-9]+),([0-9]+)\)"#).unwrap();
    let ans = regex
        .captures_iter(&input)
        .map(|cap| {
            let num1 = cap.get(1).unwrap().as_str().parse::<i64>().expect("parse");
            let num2 = cap.get(2).unwrap().as_str().parse::<i64>().expect("parse");
            num1 * num2
        })
        .sum::<i64>();

    println!("{}", ans);
}
