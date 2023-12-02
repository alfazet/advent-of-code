pub fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let ans: u32 = lines.enumerate().map(|(id, line)| {
        let mut ok = true;
        let fields: Vec<&str> = line.split(' ').collect();

        let mut i = 2;
        let (mut r, mut g, mut b) = (0, 0, 0);
        loop {
            let cnt = fields[i].parse::<u32>().expect("not an int");
            let color: Vec<char> = fields[i + 1].chars().collect();
            match color[0] {
                'r' => r += cnt,
                'g' => g += cnt,
                'b' => b += cnt,
                _ => panic!("invalid char"),
            }

            let last = *color.last().expect("empty color");
            if last != ',' {
                if r > 12 || g > 13 || b > 14 {
                    ok = false;
                }
                if last != ';' {
                    break;
                }
                (r, g, b) = (0, 0, 0);
            }
            i += 2;
        }

        if ok { id as u32 + 1 } else { 0 }
    }).sum();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solve(input), 8);
    }
}
