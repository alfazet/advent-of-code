use regex::Regex;

const ALPHA: usize = 26;

#[derive(Clone)]
enum Kind {
    Source,
    Dest,
    Normal,
}

#[derive(Clone)]
struct Node {
    left: usize,
    right: usize,
    kind: Kind,
}

pub fn solve(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut nodes = vec![Node { left: 0, right: 0, kind: Kind::Normal }; ALPHA * ALPHA * ALPHA];
    let dirs: Vec<char> = lines.nth(0).unwrap().chars().collect();

    let encode = |label: &str| -> usize {
        let (mut ans, mut b) = (0, 1);
        for c in label.chars() {
            ans += (c as u32 - 'A' as u32) as usize * b;
            b *= ALPHA;
        }
        ans as usize
    };

    let extract_codes = Regex::new(r"[A-Z]{3}").unwrap();
    let mut src = Vec::new();
    for line in lines.skip(1) {
        let matches: Vec<_> = extract_codes.find_iter(line).map(|s| s.as_str()).collect();
        let (c_me, c_l, c_r) = (encode(matches[0]), encode(matches[1]), encode(matches[2]));
        nodes[c_me] = Node { left: c_l, right: c_r, kind: 
            match matches[0].chars().nth(2).unwrap() {
                'A' => {
                    src.push(c_me);
                    Kind::Source
                },
                'Z' => Kind::Dest,
                _ => Kind::Normal,
            }
        };
    }

    let gcd = |mut a: i64, mut b: i64| -> i64 {
        while b > 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    };
    let lcm = |a: i64, b: i64| -> i64 {
        (a / gcd(a, b)) * b
    };

    let ans = src.iter().map(|&v| {
        let (mut cur, mut steps) = (v, 0);
        loop {
            match nodes[cur].kind {
                Kind::Dest => break,
                _ => (),
            }
            cur = match dirs[steps % dirs.len()] {
                'L' => nodes[cur].left,
                'R' => nodes[cur].right,
                _ => panic!("somehow this isnt L/R"),
            };
            steps += 1;
        }
        steps as i64
    }).fold(1, |a, b| lcm(a, b));

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve(input), 6);
    }
}
