use regex::Regex;

const ALPHA: usize = 26;

#[derive(Clone)]
struct Node {
    left: usize,
    right: usize,
}

pub fn solve(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut nodes = vec![Node { left: 0, right: 0 }; ALPHA * ALPHA * ALPHA];
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
    for line in lines.skip(1) {
        let matches: Vec<_> = extract_codes.find_iter(line).map(|s| s.as_str()).collect();
        let (c_me, c_l, c_r) = (encode(matches[0]), encode(matches[1]), encode(matches[2]));
        nodes[c_me] = Node { left: c_l, right: c_r };
    }

    let (mut cur, mut steps) = (0, 0);
    let dest = encode("ZZZ");
    loop {
        if cur == dest {
            break;
        }
        cur = match dirs[steps % dirs.len()] {
            'L' => nodes[cur].left,
            'R' => nodes[cur].right,
            _ => panic!("somehow this isnt L/R"),
        };
        steps += 1;
    }

    steps as u32
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
