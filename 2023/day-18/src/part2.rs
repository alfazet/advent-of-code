use regex::Regex;

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(input: &str) -> i64 {
    let lines = input.lines();
    let get_hex_code = Regex::new(r"#.{6}").unwrap();
    let mut coords: Vec<(i64, i64)> = Vec::new();
    coords.push((0, 0));
    for line in lines {
        let matches: Vec<_> = get_hex_code.find_iter(line).map(|s| s.as_str()).collect();
        let hex_code = *matches.last().unwrap();
        let dir_digit = hex_code.chars().last().unwrap();
        let dir = match dir_digit {
            '3' => Dir::Up,
            '1' => Dir::Down,
            '2' => Dir::Left,
            '0' => Dir::Right,
            _ => panic!("invalid char"),
        };
        let len = i64::from_str_radix(&hex_code[1..6], 16).expect("couldnt parse hex");

        let mut new_coord = *coords.last().unwrap();
        match dir {
            Dir::Up => new_coord.1 += len,
            Dir::Down => new_coord.1 -= len,
            Dir::Left => new_coord.0 -= len,
            Dir::Right => new_coord.0 += len,
        }
        coords.push(new_coord);
    }

    let n = coords.len();
    let mut area = 0; // actually 2 * area
    let mut bound = 0;
    for i in 0..(n - 1) { // shoelace formula
        area += coords[i].0 * coords[i + 1].1 - coords[i + 1].0 * coords[i].1;
        bound += (coords[i].0 - coords[i + 1].0).abs() + (coords[i].1 - coords[i + 1].1).abs();
    }
    area = area.abs();

    (area - bound + 2) / 2 + bound // pick's formula
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(solve(input), 952408144115);
    }
}
