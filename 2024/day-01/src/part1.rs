pub fn solve(input: String) {
    let lines = input.lines();
    let mut a: Vec<(u8, i32)> = Vec::new();
    for line in lines {
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("parse"))
            .collect::<Vec<i32>>();
        a.push((0, numbers[0]));
        a.push((1, numbers[1]));
    }
    a.sort_unstable();

    let n = a.len();
    let (first_half, second_half) = (&a[..n / 2], &a[n / 2..]);
    let ans = first_half
        .iter()
        .zip(second_half.iter())
        .map(|(x, y)| (x.1 - y.1).abs())
        .sum::<i32>();
    println!("{}", ans);
}
