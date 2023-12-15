const A: u32 = 17;
const MOD: u32 = 256;

pub fn solve(input: &str) -> u32 {
    let line = input.lines().last().unwrap();
    let ans = line.split(',').map(|step| {
        let mut h = 0;
        for c in step.chars() {
            h += c as u32;
            h *= A;
            h %= MOD;
        }

        h
    }).sum();

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve(input), 1320);
    }
}
