use std::collections::HashMap;

pub fn solve(input: String) {
    let lines = input.lines();
    let mut a: Vec<u64> = Vec::new();
    let mut map: HashMap<u64, u64> = HashMap::new();
    for line in lines {
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<u64>().expect("parse"))
            .collect::<Vec<u64>>();
        a.push(numbers[0]);
        map.entry(numbers[1]).and_modify(|v| *v += 1).or_insert(1);
    }
    let ans = a
        .into_iter()
        .map(|x| x * map.get(&x).unwrap_or(&0))
        .sum::<u64>();
    println!("{}", ans);
}
