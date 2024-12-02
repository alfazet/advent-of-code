fn check_ok(a: &[i32], ignore: usize) -> i32 {
    let n = a.len();
    let sign = (a[n - 1] - a[0]).signum();
    if sign == 0 {
        return 0;
    }
    if ignore == n + 1 {
        for i in 1..n {
            let delta = a[i] - a[i - 1];
            if delta.signum() != sign || delta.abs() < 1 || delta.abs() > 3 {
                return 0;
            }
        }
        return 1;
    }

    let mut prev = a[0];
    for (i, x) in a.iter().enumerate().take(n).skip(1) {
        if i == ignore {
            continue;
        }
        let delta = x - prev;
        if delta.signum() != sign || delta.abs() < 1 || delta.abs() > 3 {
            return 0;
        }
        prev = *x;
    }

    1
}

pub fn solve(input: String) {
    let lines = input.lines();
    let ans = lines
        .map(|line| {
            let a = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().expect("parse"))
                .collect::<Vec<_>>();
            let n = a.len();
            let mut any_ok = 0;
            any_ok |= check_ok(&a[1..], n + 1);
            for ignore in 1..(n - 1) {
                any_ok |= check_ok(&a, ignore);
            }
            any_ok |= check_ok(&a[..(n - 1)], n + 1);

            any_ok
        })
        .sum::<i32>();
    println!("{}", ans);
}
