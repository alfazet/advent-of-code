pub fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let ans: u32 = lines.map(|line| {
        let fields: Vec<&str> = line.split(' ').collect();

        let mut i = 2;
        let (mut r, mut g, mut b) = (0, 0, 0);
        loop {
            let cnt = fields[i].parse::<u32>().expect("not an int");
            let color: Vec<char> = fields[i + 1].chars().collect();
            match color[0] {
                'r' => r = r.max(cnt),
                'g' => g = g.max(cnt),
                'b' => b = b.max(cnt),
                _ => panic!("invalid char"),
            }

            let last = *color.last().expect("empty color");
            if last != ',' && last != ';' {
                break;
            }
            i += 2;
        }

        r * g * b
    }).sum();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solve(input), 2286);
    }
}
