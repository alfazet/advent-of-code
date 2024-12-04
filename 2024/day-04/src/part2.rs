fn find_crosses(grid: &[Vec<char>], pattern: &[char]) -> i64 {
    let mut ans = 0;
    let (n, p) = (grid.len(), pattern.len());
    for i in 0..=(n - p) {
        for j in 0..=(n - p) {
            let mut all_ok = true;
            for k in 0..p {
                all_ok &= pattern[k] == grid[i + k][j + k];
            }
            for k in 0..p {
                all_ok &= pattern[k] == grid[i + p - 1 - k][j + k];
            }
            if all_ok {
                ans += 1;
            }
        }
    }

    ans
}

fn rot90(grid: &mut [Vec<char>]) {
    let n = grid.len();
    for i in 0..(n / 2) {
        for j in i..(n - i - 1) {
            let temp = grid[i][j];
            grid[i][j] = grid[j][n - 1 - i];
            grid[j][n - 1 - i] = grid[n - 1 - i][n - 1 - j];
            grid[n - 1 - i][n - 1 - j] = grid[n - 1 - j][i];
            grid[n - 1 - j][i] = temp;
        }
    }
}

pub fn solve(input: String) {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut ans = 0;
    for _ in 0..4 {
        ans += find_crosses(&grid, &['M', 'A', 'S']);
        rot90(&mut grid);
    }
    println!("{}", ans);
}
