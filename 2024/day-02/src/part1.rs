pub fn solve(input: String) {
    let lines = input.lines();
    let ans = lines
        .map(|line| {
            let a = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().expect("parse"))
                .collect::<Vec<_>>();
            let n = a.len();
            let sign = (a[n - 1] - a[0]).signum();
            if sign == 0 {
                return 0;
            }
            for i in 1..n {
                let delta = a[i] - a[i - 1];
                if delta.signum() != sign || delta.abs() < 1 || delta.abs() > 3 {
                    return 0;
                }
            }
            1
        })
        .sum::<i32>();
    println!("{}", ans);
}
