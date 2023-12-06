pub fn solve(input: &str) -> i64 {
    let mut lines = input.lines();
    let t: Vec<i64> = lines.nth(0).unwrap().to_owned().split_whitespace().skip(1)
        .map(|x| x.parse::<i64>().expect("couldnt parse int")).collect();
    let d: Vec<i64> = lines.nth(0).unwrap().to_owned().split_whitespace().skip(1)
        .map(|x| x.parse::<i64>().expect("couldnt parse int")).collect();
    let ans = t.iter().zip(d.iter()).map(|(t, d)| {
        let ways = (0..100).filter(|t1| -t1 * t1 + t * t1 - d > 0).count(); 
        ways as i64
    }).product();
    
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
        assert_eq!(solve(input), 288);
    }
}
