pub mod common {
    use core::cmp::Ordering;
    use std::collections::HashMap;

    #[derive(Debug, Eq)]
    pub struct Hand {
        pub cards: Vec<char>,
        pub bit: i32,
        pub comb_rate: usize,
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            let card_score: HashMap<char, i32> = HashMap::from([
                ('A', 14),
                ('K', 13),
                ('Q', 12),
                ('J', 11),
                ('T', 10),
                ('9', 9),
                ('8', 8),
                ('7', 7),
                ('6', 6),
                ('5', 5),
                ('4', 4),
                ('3', 3),
                ('2', 2),
            ]);
            if self.comb_rate > other.comb_rate {
                return Ordering::Greater;
            }

            if self.comb_rate < other.comb_rate {
                return Ordering::Less;
            }

            for i in 0..self.cards.len() {
                if card_score[&self.cards[i]] > card_score[&other.cards[i]] {
                    return Ordering::Greater;
                }
                if card_score[&self.cards[i]] < card_score[&other.cards[i]] {
                    return Ordering::Less;
                }
            }

            return Ordering::Equal;
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.cmp(&other) == Ordering::Equal
        }
    }

    impl Hand {
        pub fn parse(line: &String) -> Hand {
            let chars: Vec<char> = line.chars().collect();
            let mut bit_str = String::new();
            let mut i = 6;

            while i < chars.len() {
                bit_str.push(chars[i]);
                i += 1;
            }

            let mut cards: Vec<char> = Vec::new();
            for i in 0..5 {
                cards.push(chars[i]);
            }
            let bit: i32 = bit_str.parse().unwrap();
            let comb_rate = Self::get_combination(&cards);

            Hand {
                cards,
                bit,
                comb_rate,
            }
        }

        fn get_combination(cards: &Vec<char>) -> usize {
            let mut m = HashMap::new();

            for c in cards {
                *m.entry(c).or_insert(0) += 1;
            }

            match m.len() {
                1 => 6,
                2 => {
                    if m[&cards[0]] == 1 || m[&cards[0]] == 4 {
                        5
                    } else {
                        4
                    }
                }
                3 => {
                    for v in m.values() {
                        if *v == 3 {
                            return 3;
                        }
                    }
                    2
                }
                4 => 1,
                _ => 0,
            }
        }
    }
}

pub mod solution1 {
    use crate::common::Hand;

    pub fn process(input: &Vec<String>) -> i64 {
        let mut hands = Vec::new();
        for line in input {
            hands.push(Hand::parse(&line));
        }
        let _ = &hands.sort();

        let mut res = 0;
        for i in 0..hands.len() {
            res += (i + 1) as i64 * hands[i].bit as i64;
        }

        return res;
    }
}


pub mod solution2 {
    use crate::common::Hand;

    pub fn process(input: &Vec<String>) -> i64 {
        let mut hands = Vec::new();
        for line in input {
            hands.push(Hand::parse(&line));
        }
        let _ = &hands.sort();

        let mut res = 0;
        for i in 0..hands.len() {
            res += (i + 1) as i64 * hands[i].bit as i64;
        }

        return res;
    }
}
#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn example_works() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(solution1::process(&input), 6440);
    }
}


#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn example_works() {
        let input = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        assert_eq!(solution2::process(&input), 5905);
    }
}
