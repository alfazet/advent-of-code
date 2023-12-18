#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn solve(input: &str) -> i32 {
    let lines = input.lines();
    let mut coords: Vec<(i32, i32)> = Vec::new();
    coords.push((0, 0));
    for line in lines {
        let mut split_line = line.split_whitespace();
        let dir = match split_line.next().unwrap() {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!("invalid char"),
        };
        let len = split_line.next().unwrap().parse::<i32>().expect("couldnt parse int");

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
        assert_eq!(solve(input), 62);
    }
}
