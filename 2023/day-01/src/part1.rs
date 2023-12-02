pub fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let ans: u32 = lines.map(|line| {
        let line_vec: Vec<char>  = line.chars().collect();
        let l = line_vec.iter().position(|c| c.is_digit(10)).expect("no digit found");
        let r = line_vec.iter().rposition(|c| c.is_digit(10)).expect("no digit found");

        10 * (line_vec[l] as u32 - '0' as u32) + (line_vec[r] as u32 - '0' as u32)
    }).sum();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "2abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(solve(input), 142);
    }
}
