use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq)]
enum Kind {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> Ordering {
        let me = match self {
            Kind::FiveKind => 7,
            Kind::FourKind => 6,
            Kind::FullHouse => 5,
            Kind::ThreeKind => 4,
            Kind::TwoPair => 3,
            Kind::OnePair => 2,
            Kind::HighCard => 1,
        };
        let him = match other {
            Kind::FiveKind => 7,
            Kind::FourKind => 6,
            Kind::FullHouse => 5,
            Kind::ThreeKind => 4,
            Kind::TwoPair => 3,
            Kind::OnePair => 2,
            Kind::HighCard => 1,
        };
        me.cmp(&him)
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    ranks: [u32; 5],
    kind: Kind,
}

impl Hand {
    fn new(ranks_str: &str) -> Hand {
        let mut ranks = [0; 5];
        for (i, c) in ranks_str.chars().enumerate() {
            ranks[i] = match c {
                'A' => 12,
                'K' => 11,
                'Q' => 10,
                'J' => 9,
                'T' => 8,
                _ => c as u32 - '0' as u32 - 2,
            };
        }

        let mut freq = [0; 13];
        for i in 0..5 {
            freq[ranks[i] as usize] += 1;
        }
        freq.sort_by(|a, b| b.cmp(&a));

        let kind = {
            if freq[0] == 5 {
                Kind::FiveKind
            } else if freq[0] == 4 { 
                Kind::FourKind
            } else if freq[0] == 3 && freq[1] == 2 {
                Kind::FullHouse
            } else if freq[0] == 3 {
                Kind::ThreeKind
            } else if freq[0] == 2 && freq[1] == 2 {
                Kind::TwoPair
            } else if freq[0] == 2 {
                Kind::OnePair
            } else {
                Kind::HighCard
            }
        };

        Hand { ranks, kind }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.kind != other.kind {
            (self.kind).cmp(&other.kind)
        } else {
            for i in 0..5 {
                if self.ranks[i] != other.ranks[i] {
                    return (self.ranks[i]).cmp(&other.ranks[i]);
                }
            }
            Ordering::Equal
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve(input: &str) -> u32 {
    let lines = input.lines();
    let mut hands: Vec<(Hand, u32)> = lines.map(|line| {
        let mut split_line = line.split_whitespace();
        let (ranks_str, bid) = (split_line.nth(0).unwrap(), split_line.nth(0).unwrap().parse::<u32>().expect("couldnt parse int"));
        (Hand::new(ranks_str), bid)
    }).collect();

    hands.sort();
    let ans = hands.iter().enumerate().map(|(i, hand)| {
        (i as u32 + 1) * hand.1
    }).sum();
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        assert_eq!(solve(input), 6440);
    }
}
