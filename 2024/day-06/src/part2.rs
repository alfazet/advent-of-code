#[derive(Debug, PartialEq)]
enum Tile {
    Free,
    Obstacle,
    Start,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Heading {
    North,
    South,
    East,
    West,
}

struct State {
    r: usize,
    c: usize,
    heading: Heading,
    oob: bool,
}

fn mutate_state(grid: &[Vec<Tile>], state: &mut State) {
    match state.heading {
        Heading::North => {
            if state.r == 0 {
                state.oob = true;
                return;
            }
            let new_r = state.r - 1;
            if grid[new_r][state.c] == Tile::Obstacle {
                state.heading = Heading::East;
            } else {
                state.r = new_r;
            }
        }
        Heading::South => {
            if state.r == grid.len() - 1 {
                state.oob = true;
                return;
            }
            let new_r = state.r + 1;
            if grid[new_r][state.c] == Tile::Obstacle {
                state.heading = Heading::West;
            } else {
                state.r = new_r;
            }
        }
        Heading::East => {
            if state.c == grid[0].len() - 1 {
                state.oob = true;
                return;
            }
            let new_c = state.c + 1;
            if grid[state.r][new_c] == Tile::Obstacle {
                state.heading = Heading::South;
            } else {
                state.c = new_c;
            }
        }
        Heading::West => {
            if state.c == 0 {
                state.oob = true;
                return;
            }
            let new_c = state.c - 1;
            if grid[state.r][new_c] == Tile::Obstacle {
                state.heading = Heading::North;
            } else {
                state.c = new_c;
            }
        }
    }
}

pub fn solve(input: String) {
    let mut grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Free,
                    '#' => Tile::Obstacle,
                    _ => Tile::Start,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();
    let (n, m) = (grid.len(), grid[0].len());

    let (start_r, start_c) = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| { row.iter().position(|tile| *tile == Tile::Start) }.map(|j| (i, j)))
        .unwrap();

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == Tile::Free {
                grid[i][j] = Tile::Obstacle;
                let mut state = State { r: start_r, c: start_c, heading: Heading::North, oob: false };
                let mut vis = vec![vec![vec![false; 4]; m]; n];
                ans += loop {
                    let (r, c) = (state.r, state.c);
                    let h = match state.heading {
                        Heading::North => 0,
                        Heading::South => 1,
                        Heading::East => 2,
                        Heading::West => 3,
                    };
                    if !vis[r][c][h] {
                        vis[r][c][h] = true;
                    } else {
                        break 1;
                    }
                    mutate_state(&grid, &mut state);
                    if state.oob {
                        break 0;
                    }
                };
                grid[i][j] = Tile::Free;
            }
        }
    }
    println!("{}", ans);
}
