const A: usize = 17;
const MOD: usize = 256;

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_len: usize,
}

pub fn solve(input: &str) -> u32 {
    let get_hash = |s: &str| -> usize {
        let mut h = 0;
        for c in s.chars() {
            h += c as usize;
            h *= A;
            h %= MOD;
        }

        h as usize
    };

    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); MOD];
    let line = input.lines().last().unwrap();
    for step in line.split(',') {
        let op = step.chars().find(|c| !c.is_ascii_lowercase()).expect("no hyphen or equals");
        let op_pos = step.chars().position(|c| c == op).expect("no hyphen or equals");
        let the_box = &mut boxes[get_hash(&step[..op_pos])];
        let label = step[..op_pos].to_owned();

        match op {
            '-' => {
                the_box.retain(|lens| lens.label != label);
            },
            '=' => {
                let focal_len: usize = step.chars().last().unwrap().to_digit(10).expect("should be a digit").try_into().unwrap();
                if let None = the_box.iter().find(|lens| lens.label == label) {
                    the_box.push(Lens { label, focal_len });
                } else {
                    let i = the_box.iter().position(|lens| lens.label == label).expect("shouldve found it");
                    the_box[i].focal_len = focal_len;
                }
            },
            _ => (),
        }
    }

    let mut ans = 0;
    for i in 0..MOD {
        for j in 0..boxes[i].len() {
            ans += (i + 1) * (j + 1) * boxes[i][j].focal_len;
        }
    }

    ans as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve(input), 145);
    }
}
