#[derive(Debug)]
enum Field {
    Galaxy,
    Empty,
}

pub fn solve(input: &str) -> i64 {
    let lines = input.lines();
    let mut cnt_galaxies = 0;
    let a: Vec<Vec<Field>> = lines.map(|line| {
        line.chars().map(|c| {
            match c {
                '.' => Field::Empty,
                '#' => { cnt_galaxies += 1; Field::Galaxy },
                _ => panic!("invalid char"),
            }
        }).collect()
    }).collect();

    let (n, m) = (a.len(), a[0].len());
    let (mut r_expand, mut c_expand) = (Vec::new(), Vec::new());
    for r in 0..n {
        let mut is_empty = true;
        for c in 0..m {
            if let Field::Galaxy = a[r][c] {
                is_empty = false;
            }
        }
        r_expand.push(if is_empty { 999_999 } else { 0 }); 
    }
    for c in 0..m {
        let mut is_empty = true;
        for r in 0..n {
            if let Field::Galaxy = a[r][c] {
                is_empty = false;
            }
        }
        c_expand.push(if is_empty { 999_999 } else { 0 }); 
    }

    let (mut total_dist_r, mut total_dist_c) = (0, 0);
    let mut passed = 0;
    for r in 0..n {
        total_dist_r += passed * (1 + r_expand[r]) * (cnt_galaxies - passed);
        for c in 0..m {
            if let Field::Galaxy = a[r][c] {
                passed += 1;
            }
        }
    }
    passed = 0;
    for c in 0..m {
        total_dist_c += passed * (1 + c_expand[c]) * (cnt_galaxies - passed);
        for r in 0..n {
            if let Field::Galaxy = a[r][c] {
                passed += 1;
            }
        }
    }

    total_dist_r + total_dist_c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        /* 
           note: this test FAILS because the value in solve()
           is hardcoded to match the one in the proper input file (10^6 instead of 100)
        */
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(solve(input), 8410);
    }
}
