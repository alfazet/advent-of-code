use std::collections::HashSet;

pub fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let ans: u32 = lines.map(|line| {
        let fields: Vec<&str> = line.split_whitespace().skip(2).collect();
        let mut winning = HashSet::<u32>::new();
        let mut score = 0;
        let mut after = false;
        for field in fields {
            match &field {
                &"|" => {
                    after = true;
                },
                _ => { 
                    let x = field.parse::<u32>().expect("should be an int");
                    if after {
                        if winning.contains(&x) {
                            score = if score == 0 { 1 } else { 2 * score };
                        }
                    } else {
                        winning.insert(x);
                    }
                },
            }
        }
        score
    }).sum();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(solve(input), 13);
    }
}
