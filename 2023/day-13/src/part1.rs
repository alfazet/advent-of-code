#[derive(Debug)]
enum Tile {
    Ash,
    Rocks,
}

pub fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let mut patterns: Vec<Vec<Vec<Tile>>> = Vec::new();

    let mut cur: Vec<Vec<Tile>> = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            patterns.push(cur);
            cur = Vec::new();
        } else {
            let parsed: Vec<Tile> = line.chars().map(|c| {
                match c {
                    '.' => Tile::Ash,
                    '#' => Tile::Rocks,
                    _ => panic!("invalid char"),
                }
            }).collect();
            cur.push(parsed);
        }
    }
    patterns.push(cur);

    let ans = patterns.iter().map(|pattern| {
        let (n, m) = (pattern.len(), pattern[0].len());
        let (mut r_hashes, mut c_hashes) = (Vec::new(), Vec::new());
        for r in 0..n {
            let (mut cur, mut p) = (0, 1);
            for c in 0..m {
                if let Tile::Rocks = pattern[r][c] {
                    cur += p;
                }
                p *= 2;
            }
            r_hashes.push(cur);
        }
        for c in 0..m {
            let (mut cur, mut p) = (0, 1);
            for r in 0..n {
                if let Tile::Rocks = pattern[r][c] {
                    cur += p;
                }
                p *= 2;
            }
            c_hashes.push(cur);
        }

        let (mut row_sym_ans, mut col_sym_ans) = (0, 0);
        for c in 0..m {
            let mut i = 0;
            loop {
                if c - i < 1 || c + i > m - 1 {
                    break;
                }
                if c_hashes[c - i - 1] != c_hashes[c + i] {
                    i = usize::MAX;
                    break;
                }
                i += 1;
            }
            if i != usize::MAX {
                col_sym_ans += c;
            }
        }
        for r in 0..n {
            let mut i = 0;
            loop {
                if r - i < 1 || r + i > n - 1 {
                    break;
                }
                if r_hashes[r - i - 1] != r_hashes[r + i] {
                    i = usize::MAX;
                    break;
                }
                i += 1;
            }
            if i != usize::MAX {
                row_sym_ans += 100 * r;
            }
        }

        (row_sym_ans + col_sym_ans) as u32
    }).sum();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(solve(input), 405);
    }
}
