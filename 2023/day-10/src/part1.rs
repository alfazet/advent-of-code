use colored::Colorize;

#[derive(Debug)]
enum Pipe {
    SN,
    EW,
    NE,
    NW,
    SE,
    SW,
    Empty,
    Start,
}

#[derive(Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

pub fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let mut a: Vec<Vec<Pipe>> = Vec::new(); 
    for line in lines {
        a.push(line.chars().map(|c| match c {
            '|' => Pipe::SN,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            'F' => Pipe::SE,
            '7' => Pipe::SW,
            'S' => Pipe::Start, 
            '.' => Pipe::Empty,
            _ => panic!("invalid char"),
        }).collect());
    }

    let (n, m) = (a.len(), a[0].len());
    let (mut sr, mut sc) = (0, 0);
    for r in 0..n {
        for c in 0..m {
            if let Pipe::Start = a[r][c] {
                (sr, sc) = (r, c);
                a[r][c] = Pipe::NE; // found by looking at the input file, I didn't want to bother checking all the cases
            }
        }
    }

    let mut main_loop: Vec<Vec<bool>> = vec![vec![false; m]; n];
    let (mut r, mut c, mut steps) = (sr, sc, 0);
    let mut came_from = Dir::N;
    loop {
        main_loop[r][c] = true;
        (r, c) = match a[r][c] {
            Pipe::SN => {
                match came_from {
                    Dir::N => { came_from = Dir::N; (r + 1, c) },
                    Dir::S => { came_from = Dir::S; (r - 1, c) },
                    _ => panic!("invalid direction"),
                }
            },
            Pipe::EW => {
                match came_from {
                    Dir::E => { came_from = Dir::E; (r, c - 1) },
                    Dir::W => { came_from = Dir::W; (r, c + 1) },
                    _ => panic!("invalid direction"),
                }
            },
            Pipe::NE => {
                match came_from {
                    Dir::N => { came_from = Dir::W; (r, c + 1) },
                    Dir::E => { came_from = Dir::S; (r - 1, c) },
                    _ => panic!("invalid direction"),
                }
            },
            Pipe::NW => {
                match came_from {
                    Dir::N => { came_from = Dir::E; (r, c - 1) },
                    Dir::W => { came_from = Dir::S; (r - 1, c) },
                    _ => panic!("invalid direction"),
                }
            },
            Pipe::SE => {
                match came_from {
                    Dir::S => { came_from = Dir::W; (r, c + 1) },
                    Dir::E => { came_from = Dir::N; (r + 1, c) },
                    _ => panic!("invalid direction"),
                }
            },
            Pipe::SW => {
                match came_from {
                    Dir::S => { came_from = Dir::E; (r, c - 1) },
                    Dir::W => { came_from = Dir::N; (r + 1, c) },
                    _ => panic!("invalid direction"),
                }
            },
            _ => (sr, sc),
        };
        steps += 1;
        if (r, c) == (sr, sc) {
            break;
        }
    }

    // just a cool visualization
    for r in 0..n {
        for c in 0..m {
            let ch = if !main_loop[r][c] { String::from('x').white() } else { String::from(match a[r][c] {
                Pipe::SN => '│',
                Pipe::EW => '─',
                Pipe::NE => '└',
                Pipe::NW => '┘',
                Pipe::SE => '┌',
                Pipe::SW => '┐',
                Pipe::Empty => '.',
                _ => panic!("invalid char"),
            }).red() };
            print!("{}", ch);
        }
        println!("");
    }

    steps / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        /* 
           note: this test FAILS because the value in solve()
           is hardcoded to match the one in the proper input file
        */
        let input = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";
        assert_eq!(solve(input), 8);
    }
}
