use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Type {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPairs = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    type_p1: Type,
    type_p2: Type,
}

impl Hand {
    fn compute_type(highest: u8, pairs: u8, jokers: u8) -> Type {
        let tup = (highest, pairs, jokers);
        if highest + jokers == 5 {
            Type::FiveOfAKind
        } else if highest + jokers == 4 {
            Type::FourOfAKind
        } else if tup == (3, 1, 0) || tup == (2, 2, 1) {
            Type::FullHouse
        } else if tup == (3, 0, 0) || tup == (1, 0, 2) || tup == (2, 1, 1) {
            Type::ThreeOfAKind
        } else if tup == (2, 2, 0) {
            Type::TwoPairs
        } else if tup == (2, 1, 0) || tup == (1, 0, 1) {
            Type::OnePair
        } else {
            Type::HighCard
        }
    }

    fn from_bytes(bytes: [u8; 5]) -> Self {
        let cards = bytes.map(|byte| {
            if (b'2'..=b'9').contains(&byte) {
                return byte - b'2';
            } else {
                for (i, b) in [b'T', b'J', b'Q', b'K', b'A'].iter().enumerate() {
                    if byte == *b {
                        return (i + 8) as u8;
                    }
                }
            }
            unreachable!()
        });
        let mut counts = [0u8; 13];
        for card in cards.iter() {
            counts[*card as usize] += 1;
        }
        let jokers = counts[9];
        let pairs_p1 = counts.iter().filter(|&&s| s == 2).count() as u8;
        counts[9] = 0;
        let pairs_p2 = pairs_p1 - (jokers == 2) as u8;
        let highest_p2 = *counts.iter().max().unwrap();
        let highest_p1 = highest_p2.max(jokers);
        Hand {
            cards,
            type_p1: Hand::compute_type(highest_p1, pairs_p1, 0),
            type_p2: Hand::compute_type(highest_p2, pairs_p2, jokers),
        }
    }

    fn cmp_p1(&self, other: &Self) -> Ordering {
        match self.type_p1.cmp(&other.type_p1) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            x => x,
        }
    }

    fn cmp_p2(&self, other: &Self) -> Ordering {
        match self.type_p2.cmp(&other.type_p2) {
            Ordering::Equal => {
                let mapper = |v: &[u8; 5]| v.map(|i| if i == 9 { 0 } else { i + 1 });
                mapper(&self.cards).cmp(&mapper(&other.cards))
            }
            x => x,
        }
    }
}

fn parse(s: &str) -> Vec<(Hand, usize)> {
    s.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|line| {
            let (h, n) = line.split_once(' ').unwrap();
            (
                Hand::from_bytes(h.as_bytes().try_into().unwrap()),
                n.parse().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

pub fn solve(s: &str) -> (usize, usize) {
    let mut hands = parse(s);
    let get_order = |v: &[(Hand, usize)]| v.iter().enumerate().map(|(i, (_, n))| (i + 1) * n).sum();
    hands.sort_unstable_by(|a, b| Hand::cmp_p1(&a.0, &b.0));
    let p1 = get_order(&hands);
    hands.sort_unstable_by(|a, b| Hand::cmp_p2(&a.0, &b.0));
    let p2 = get_order(&hands);
    (p1, p2)
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (6440, 5905));
    }
}
