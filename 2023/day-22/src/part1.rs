use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Brick {
    x1: u32,
    y1: u32,
    z1: u32,
    x2: u32,
    y2: u32,
    z2: u32,
}

pub fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let mut sz = 0;
    let mut n = 0;
    let mut bricks: Vec<Brick> = lines.map(|line| {
        let p1 = line.split('~').nth(0).unwrap();
        let p1 = p1.split(',').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let p2 = line.split('~').nth(1).unwrap();
        let p2 = p2.split(',').map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        sz = sz.max(p1[0]).max(p1[1]).max(p2[0]).max(p2[1]).max(p1[2]).max(p2[2]);
        n += 1;
        Brick { x1: p1[0], y1: p1[1], z1: p1[2], x2: p2[0], y2: p2[1], z2: p2[2] }
    }).collect();
    let mut ans = HashSet::new();

    bricks.sort_by(|a, b| a.z1.cmp(&b.z1));
    let mut ground = vec![vec![0; sz as usize]; sz as usize];
    let mut bricks_ids = vec![vec![0; sz as usize]; sz as usize];
    for (i, brick) in bricks.iter().enumerate() {
        let mut z_after_fall = 0; // the z coord that the brick will fall on
        let mut falls_on = Vec::new(); // ids of the bricks that this brick will fall on
        for x in brick.x1..=brick.x2 {
            for y in brick.y1..=brick.y2 {
                let (x, y) = (x as usize, y as usize);
                z_after_fall = z_after_fall.max(ground[x][y]);
            }
        }

        for x in brick.x1..=brick.x2 {
            for y in brick.y1..=brick.y2 {
                let (x, y) = (x as usize, y as usize);
                if bricks_ids[x][y] != 0 && ground[x][y] == z_after_fall {
                    falls_on.push(bricks_ids[x][y]);
                }
            }
        }

        let falls_on_one = !falls_on.is_empty() && falls_on.iter().all(|&x| x == falls_on[0]);
        if falls_on_one {
            ans.insert(falls_on[0]);
        }

        z_after_fall += brick.z2 - brick.z1 + 1;
        for x in brick.x1..=brick.x2 {
            for y in brick.y1..=brick.y2 {
                let (x, y) = (x as usize, y as usize);
                ground[x][y] = z_after_fall;
                bricks_ids[x][y] = i + 1;
            }
        }
    }

    (n - ans.len()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(solve(input), 5);
    }
}
