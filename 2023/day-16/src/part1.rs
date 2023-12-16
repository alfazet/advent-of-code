#[derive(Debug)]
enum Tile {
    Empty,
    MirrorR, // "slash", a west->east beam is reflected into south->north
    MirrorL, // "backslash", a west->east beam is reflected into north->south
    SplitV, // "pipe"
    SplitH, // "hyphen"
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    North = 0,
    South = 1,
    East = 2,
    West = 3,
}
const NORTH: usize = 0;
const SOUTH: usize = 1;
const EAST: usize = 2;
const WEST: usize = 3;

struct State {
    r: usize,
    c: usize,
    dir: Dir,
}

pub fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let a: Vec<Vec<Tile>> = lines.map(|line| {
        line.chars().map(|c| {
            match c {
                '.' => Tile::Empty,
                '/' => Tile::MirrorR,
                '\\' => Tile::MirrorL,
                '|' => Tile::SplitV,
                '-' => Tile::SplitH,
                _ => panic!("invalid char"),
            }
        }).collect()
    }).collect();
    let (n, m) = (a.len(), a[0].len());

    let mut q: Vec<State> = Vec::new();
    q.push(State { r: 0, c: 0, dir: Dir::South });
    let mut vis = vec![vec![vec![false; 4]; m]; n];

    while let Some(v) = q.pop() {
        let (r, c, dir) = (v.r, v.c, v.dir);
        let dir_id = dir as usize;
        vis[r][c][dir_id] = true;
        match a[r][c] {
            Tile::Empty => {
                match dir {
                    Dir::North => {
                        if r > 0 && !vis[r - 1][c][dir_id] {
                            q.push(State { r: r - 1, c, dir });
                        }
                    },
                    Dir::South => {
                        if r < n - 1 && !vis[r + 1][c][dir_id] {
                            q.push(State { r: r + 1, c, dir });
                        }
                    },
                    Dir::East => {
                        if c < m - 1 && !vis[r][c + 1][dir_id] {
                            q.push(State { r, c: c + 1, dir });
                        }
                    },
                    Dir::West => {
                        if c > 0 && !vis[r][c - 1][dir_id] {
                            q.push(State { r, c: c - 1, dir });
                        }
                    },
                }
            },
            Tile::MirrorR => {
                match dir {
                    Dir::North => {
                        if c < m - 1 && !vis[r][c + 1][EAST] {
                            q.push(State { r, c: c + 1, dir: Dir::East });
                        }
                    },
                    Dir::South => {
                        if c > 0 && !vis[r][c - 1][WEST] {
                            q.push(State { r, c: c - 1, dir: Dir::West });
                        }
                    },
                    Dir::East => {
                        if r > 0 && !vis[r - 1][c][NORTH] {
                            q.push(State { r: r - 1, c, dir: Dir::North });
                        }
                    },
                    Dir::West => {
                        if r < n - 1 && !vis[r + 1][c][SOUTH] {
                            q.push(State { r: r + 1, c, dir: Dir::South });
                        }
                    },
                }
            },
            Tile::MirrorL => {
                match dir {
                    Dir::North => {
                        if c > 0 && !vis[r][c - 1][WEST] {
                            q.push(State { r, c: c - 1, dir: Dir::West });
                        }
                    },
                    Dir::South => {
                        if c < m - 1 && !vis[r][c + 1][EAST] {
                            q.push(State { r, c: c + 1, dir: Dir::East });
                        }
                    },
                    Dir::East => {
                        if r < n - 1 && !vis[r + 1][c][SOUTH] {
                            q.push(State { r: r + 1, c, dir: Dir::South });
                        }
                    },
                    Dir::West => {
                        if r > 0 && !vis[r - 1][c][NORTH] {
                            q.push(State { r: r - 1, c, dir: Dir::North });
                        }
                    },
                }
            },
            Tile::SplitV => {
                match dir {
                    Dir::North => {
                        if r > 0 && !vis[r - 1][c][dir_id] {
                            q.push(State { r: r - 1, c, dir });
                        }
                    },
                    Dir::South => {
                        if r < n - 1 && !vis[r + 1][c][dir_id] {
                            q.push(State { r: r + 1, c, dir });
                        }
                    },
                    Dir::East | Dir::West => {
                        if r > 0 && !vis[r - 1][c][NORTH] {
                            q.push(State { r: r - 1, c, dir: Dir::North });
                        }
                        if r < n - 1 && !vis[r + 1][c][SOUTH] {
                            q.push(State { r: r + 1, c, dir: Dir::South });
                        }
                    },
                }
            },
            Tile::SplitH => {
                match dir {
                    Dir::North | Dir::South => {
                        if c < m - 1 && !vis[r][c + 1][EAST] {
                            q.push(State { r, c: c + 1, dir: Dir::East });
                        }
                        if c > 0 && !vis[r][c - 1][WEST] {
                            q.push(State { r, c: c - 1, dir: Dir::West });
                        }
                    },
                    Dir::East => {
                        if c < m - 1 && !vis[r][c + 1][dir_id] {
                            q.push(State { r, c: c + 1, dir });
                        }
                    },
                    Dir::West => {
                        if c > 0 && !vis[r][c - 1][dir_id] {
                            q.push(State { r, c: c - 1, dir });
                        }
                    },
                }
            },
        }
    }

    let mut ans = 0;
    for r in 0..n {
        for c in 0..m {
            if vis[r][c].iter().any(|&x| x) {
                ans += 1;
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        assert_eq!(solve(input), 46);
    }
}
