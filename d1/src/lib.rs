pub mod solution1 {
    fn calibration_value(line: &str) -> (i32, i32) {
        let chars: Vec<char> = line.to_string().chars().collect();

        let mut i = 0;
        let mut j = chars.len() - 1;

        while !chars[i].is_numeric() {
            i += 1
        }
        while !chars[j].is_numeric() {
            j -= 1
        }

        return (
            chars[i].to_string().parse().unwrap(),
            chars[j].to_string().parse().unwrap(),
        );
    }

    pub fn sum_of_colibration_values(input: &Vec<String>) -> i32 {
        let mut sum = 0;

        for line in input {
            let (d1, d2) = calibration_value(&line);
            sum += d1 * 10;
            sum += d2;
        }

        return sum;
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn example_works_1() {
        let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .iter()
            .map(|&s| s.into())
            .collect();

        let sum = solution1::sum_of_colibration_values(&input);

        assert_eq!(sum, 142);
    }
}

pub mod solution2 {
    use std::collections::HashMap;

    struct Solution2 {
        convert_map: HashMap<String, i32>,
        words_forward: Vec<String>,
        words_backward: Vec<String>,
    }

    impl Solution2 {
        fn new() -> Solution2 {
            let convert_map: HashMap<String, i32> = HashMap::from([
                ("one".to_string(), 1),
                ("two".into(), 2),
                ("three".into(), 3),
                ("four".into(), 4),
                ("five".into(), 5),
                ("six".into(), 6),
                ("seven".into(), 7),
                ("eight".into(), 8),
                ("nine".into(), 9),
            ]);

            let words_forward: Vec<String> = vec![
                "one", "two", "six", "four", "five", "nine", "three", "seven", "eight",
            ]
            .iter()
            .map(|&s| s.into())
            .collect();

            let words_backward: Vec<String> = words_forward
                .clone()
                .iter_mut()
                .map(|s| s.chars().rev().collect::<String>())
                .collect();

            return Solution2 {
                convert_map,
                words_forward,
                words_backward,
            };
        }

        /// words are sorted by length
        /// Returns Some(index of found word) or None
        fn try_find_any_word(
            &self,
            chars: &Vec<char>,
            start_pos: usize,
            forward: bool,
            words: &Vec<String>,
        ) -> Option<usize> {
            let mut i: i32 = start_pos as i32;
            let delta: i32 = if forward { 1 } else { -1 };

            let mut candidate: String = String::new();
            candidate.push(chars[i as usize]);
            i += delta;
            candidate.push(chars[i as usize]);

            i += delta;
            if i >= chars.len() as i32 || i < 0 {
                return None;
            }
            candidate.push(chars[i as usize]);

            let mut j = 0;
            // Try to find match in 3-lengts words
            while j < words.len() && words[j].len() == 3 {
                if words[j] == candidate {
                    return Some(j);
                } else {
                    j += 1;
                }
            }

            // Try to find match in 4-lengts words

            i += delta;

            if i >= chars.len() as i32 || i < 0 {
                return None;
            }

            candidate.push(chars[i as usize]);

            while j < words.len() && words[j].len() == 4 {
                if words[j] == candidate {
                    return Some(j);
                } else {
                    j += 1;
                }
            }

            // Try to find match in 5-lengts words

            i += delta;

            if i >= chars.len() as i32 || i < 0 {
                return None;
            }

            candidate.push(chars[i as usize]);
            while j < words.len() && words[j].len() == 5 {
                if words[j] == candidate {
                    return Some(j);
                } else {
                    j += 1;
                }
            }

            return None;
        }

        fn try_lookup_forward(&self, chars: &Vec<char>, start_pos: usize) -> Option<i32> {
            if chars[start_pos].is_numeric() {
                return Some(chars[start_pos].to_string().parse().expect("Numeric err"));
            }

            return match self.try_find_any_word(chars, start_pos, true, &self.words_forward) {
                Some(ind) => Some(*self.convert_map.get(&self.words_forward[ind]).unwrap()),
                None => None,
            };
        }

        fn try_lookup_backward(&self, chars: &Vec<char>, start_pos: usize) -> Option<i32> {
            if chars[start_pos].is_numeric() {
                return Some(chars[start_pos].to_string().parse().expect("Parse err"));
            }

            return match self.try_find_any_word(chars, start_pos, false, &self.words_backward) {
                Some(ind) => Some(
                    *self
                        .convert_map
                        .get(
                            &self.words_backward[ind]
                                .clone()
                                .chars()
                                .rev()
                                .collect::<String>(),
                        )
                        .expect("Convertion err"),
                ),

                None => None,
            };
        }
    }

    fn calibration_value(line: &str) -> (i32, i32) {
        let chars: Vec<char> = line.to_string().chars().collect();

        let solution2 = Solution2::new();

        let mut i = 0;
        let mut j = chars.len() - 1;
        let mut d1 = 0;
        let mut d2 = 0;

        loop {
            match &solution2.try_lookup_forward(&chars, i) {
                Some(v) => {
                    d1 = *v;
                    break;
                }
                None => i += 1,
            }
        }

        loop {
            match &solution2.try_lookup_backward(&chars, j) {
                Some(v) => {
                    d2 = *v;
                    break;
                }
                None => j -= 1,
            }
        }

        return (d1, d2);
    }

    pub fn sum_of_colibration_values(input: &Vec<String>) -> i32 {
        let mut sum = 0;

        for line in input {
            let (d1, d2) = calibration_value(&line);
            sum += d1 * 10;
            sum += d2;
        }

        return sum;
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn example_works_2() {
        let input = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        let sum = solution2::sum_of_colibration_values(&input);

        assert_eq!(281, sum);
    }
}
