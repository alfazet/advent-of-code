#[derive(Debug, PartialEq)]
enum Tile {
    Free,
    Obstacle,
    Start,
}

#[derive(Debug)]
enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct State {
    r: usize,
    c: usize,
    heading: Heading,
    oob: bool,
}

impl State {
    fn new(r: usize, c: usize, heading: Heading, oob: bool) -> Self {
        State { r, c, heading, oob }
    }
}

pub fn solve(input: String) {
    let grid = input
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

    let mutate_state = |state: &mut State| match state.heading {
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
            if state.r == n - 1 {
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
            if state.c == m - 1 {
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
    };

    let (r, c) = grid
        .iter()
        .enumerate()
        .find_map(|(i, row)| { row.iter().position(|tile| *tile == Tile::Start) }.map(|j| (i, j)))
        .unwrap();

    let mut state = State::new(r, c, Heading::North, false);
    let mut ans = 0;
    let mut vis = vec![vec![false; m]; n];
    loop {
        let (r, c) = (state.r, state.c);
        if !vis[r][c] {
            vis[r][c] = true;
            ans += 1;
        }
        mutate_state(&mut state);
        if state.oob {
            break;
        }
    }
    println!("{}", ans);
}
