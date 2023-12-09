pub fn solve(input: &str) -> i32 {
    let lines = input.lines();
    let ans: i32 = lines.map(|line| {
        let first: Vec<i32> = line.split_whitespace()
            .map(|x| x.parse::<i32>().expect("couldnt parse int")).rev().collect();
        let mut a: Vec<Vec<i32>> = Vec::new();
        a.push(first);
        loop {
            let last = a.last().unwrap();
            let diffs: Vec<i32> = last.windows(2).map(|window| window[1] - window[0]).collect();
            if diffs.iter().all(|&x| x == 0) {
                break;
            }
            a.push(diffs);
        }
        
        let k = a.len();
        let mut new_one = a[k - 1][0];
        a[k - 1].push(new_one);
        for i in (0..(k - 1)).rev() {
            new_one = a[i][a[i].len() - 1] + a[i + 1][a[i + 1].len() - 1];
            a[i].push(new_one);
        }

        *a[0].last().unwrap()
    }).sum();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
        assert_eq!(solve(input), 2);
    }
}
