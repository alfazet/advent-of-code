fn num_len(mut x: i64) -> i64 {
    let mut ans = 0;
    while x > 0 {
        ans += 1;
        x /= 10;
    }
    ans
}

pub fn solve(input: &str) -> i64 {
    let mut lines = input.lines();
    let t_vec: Vec<i64> = lines.nth(0).unwrap().to_owned().split_whitespace().skip(1)
        .map(|x| x.parse::<i64>().expect("couldnt parse int")).collect();
    let d_vec: Vec<i64> = lines.nth(0).unwrap().to_owned().split_whitespace().skip(1)
        .map(|x| x.parse::<i64>().expect("couldnt parse int")).collect();

    let mut t = 0; // there HAS to be a better way to combine all the digits into one number, but alas
    for x in t_vec {
        let mut p = 1;
        for _ in 0..num_len(x) {
            p *= 10;
        }
        t *= p;
        t += x;
    }

    let mut d = 0;
    for x in d_vec {
        let mut p = 1;
        for _ in 0..num_len(x) {
            p *= 10;
        }
        d *= p;
        d += x;
    }

    let mut ans = 0;
    let vertex = t / 2;
    let mut i = 0;
    loop {
        let t1 = vertex - i;
        if -t1 * t1 + t * t1 - d <= 0 {
            break;
        }
        ans += 1;
        i += 1;
    }
    i = 1;
    loop {
        let t1 = vertex + i;
        if -t1 * t1 + t * t1 - d <= 0 {
            break;
        }
        ans += 1;
        i += 1;
    }

    ans
}

// t1 = loading
// t2 = driving
// t = t1 + t2 -> t2 = t - t1
// we need t2 * t1 > d
// (t - t1) * t1 > d
// - t1^2 + t * t1 - d > 0
// how many non-negative integers t1 satisfy this inequality?

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        assert_eq!(solve(input), 71503);
    }
}
