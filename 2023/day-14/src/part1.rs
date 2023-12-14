#[derive(Debug, Clone)]
enum Rock {
    Round,
    Square,
    Empty,
}

pub fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let mut a: Vec<Vec<Rock>> = lines.map(|line| {
        line.chars().map(|c| {
            match c {
                'O' => Rock::Round,
                '#' => Rock::Square,
                '.' => Rock::Empty,
                _ => panic!("invalid char"),
            }
        }).collect::<Vec<_>>()
    }).collect();
    let m = a[0].len();
    a.insert(0, vec![Rock::Square; m]); 
    let n = a.len();

    let mut rocks_below = vec![vec![0; m]; n];
    for c in 0..m {
        let mut cur = 0;
        for r in (0..n).rev() {
            match a[r][c] {
                Rock::Round => {
                    cur += 1;
                },
                Rock::Square => {
                    rocks_below[r][c] = cur;
                    cur = 0;
                },
                _ => (),
            }
        }
    }
    for r in 0..n {
        for c in 0..m {
            if let Rock::Round = a[r][c] {
                a[r][c] = Rock::Empty;
            }
        }
    }

    let mut ans = 0;
    for r in 0..n {
        for c in 0..m {
            for i in 1..=rocks_below[r][c] {
                a[r + i][c] = Rock::Round;
                ans += n - (r + i);
            }
        }
    }

    ans as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(solve(input), 136);
    }
}
