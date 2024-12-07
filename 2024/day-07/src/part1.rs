fn check(rhs: &[i64], mask: u16) -> i64 {
    let mut ans = rhs[0];
    for (i, x) in rhs.iter().enumerate().skip(1) {
        if mask & (1 << (i - 1)) == 0 {
            ans += x;
        } else {
            ans *= x;
        }
    }
    ans
}

pub fn solve(input: String) {
    let ans = input
        .lines()
        .map(|line| {
            let (val, rhs_str) = line
                .split_once(':')
                .map(|(val, rhs)| (val.parse::<i64>().expect("parse"), rhs.trim_start()))
                .unwrap();
            let rhs = rhs_str
                .split_whitespace()
                .map(|x| x.parse::<i64>().expect("parse"))
                .collect::<Vec<i64>>();

            let n = rhs.len();
            for mask in 0..(1 << (n - 1)) {
                if check(&rhs, mask) == val {
                    return val;
                }
            }
            0
        })
        .sum::<i64>();
    println!("{}", ans);
}
