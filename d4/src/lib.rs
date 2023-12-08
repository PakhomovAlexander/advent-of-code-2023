pub mod solution1 {
    struct Card {
        id: i32,
        win_nums: std::collections::HashSet::<i32>,
        my_nums: Vec<i32>,
    }

    impl Card {
        pub fn parse(line: &str) -> Card {
            let chars: Vec<char> = line.chars().collect();

            let mut id = -1;
            let mut win_nums = std::collections::HashSet::new();
            let mut my_nums = Vec::new();

            let mut i = 0;
            // find id
            while !chars[i].is_numeric() {
                i += 1;
            }
            // read id
            let mut id_str = String::new();
            while chars[i].is_numeric() {
                id_str.push(chars[i]);
                i += 1;
            }
            id = id_str.parse::<i32>().unwrap();

            i += 1; // skip ':'

            // read win nums
            while chars[i] != '|' {
                let mut curr_num_str = String::new();
                while chars[i].is_numeric() {
                    curr_num_str.push(chars[i]);
                    i += 1;
                }

                if curr_num_str.len() > 0 {
                    win_nums.insert(curr_num_str.parse::<i32>().unwrap());
                }

                while chars[i] == ' ' {
                    i += 1;
                }
            }

            i += 1; // skip |
            // read my nums
            while i < chars.len() {
                let mut curr_num_str = String::new();
                while i < chars.len() && chars[i].is_numeric() {
                    curr_num_str.push(chars[i]);
                    i += 1;
                }

                if curr_num_str.len() > 0 {
                    my_nums.push(curr_num_str.parse::<i32>().unwrap());
                }

                while i < chars.len() && chars[i] == ' ' {
                    i += 1;
                }
            }

            Card {
                id,
                win_nums,
                my_nums,
            }
        }

        pub fn score(&self) -> i32 {
            let mut score: i32 = -1;
            for n in &self.my_nums {
                if self.win_nums.contains(&n) {
                    score += 1;
                }
            }

            if score == -1 {
                0
            } else {
                i32::pow(2, score as u32)
            }
        }
    }

    pub fn process(input: &Vec<String>) -> i32 {
        let mut sum = 0;
        for line in input {
            sum += Card::parse(&line).score();
        }

        sum
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn example_works() {
        let input = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        assert_eq!(13, solution1::process(&input));
    }
}
