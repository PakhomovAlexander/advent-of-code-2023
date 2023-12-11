pub mod common {
    #[derive(Debug)]
    pub struct Cursor<'a> {
        pub input: &'a Vec<String>,
        pub top_line: Vec<char>,
        pub current_line: Vec<char>,
        pub bottom_line: Vec<char>,
        pub current_row_ind: usize,
        pub skip_characters: Vec<char>,
    }

    impl Cursor<'_> {
        pub fn init(input: &Vec<String>) -> Cursor {
            assert!(input.len() > 1);

            let skip_characters = vec!['.'];

            let current_row_ind = 0;

            let bottom_line: Vec<char> = input[1].chars().collect();
            let current_line: Vec<char> = input[0].chars().collect();
            // fill the virtual first top line with dots
            let top_line: Vec<char> = vec!['.'; bottom_line.len()];

            Cursor {
                input,
                top_line,
                current_line,
                bottom_line,
                current_row_ind,
                skip_characters,
            }
        }

        fn height(&self) -> usize {
            self.input.len()
        }

        fn width(&self) -> usize {
            self.current_line.len()
        }

        // Before the call:
        //
        // ..11 <-- top_line
        // .0.. <-- current_line (current_row_ind = 1)
        // #... <-- bottom_line
        // ..1.
        //
        // After the call:
        //
        // ..11
        // .0.. <-- top_line
        // #... <-- current_line (current_row_ind = 2)
        // ..1. <-- bottom_line
        //
        // Returns false if the cursor is closed (curent_line points at the last row).
        pub fn move_next(&mut self) -> bool {
            // Cursor points at the last line.
            if self.current_row_ind == self.height() - 1 {
                return false;
            }

            self.current_row_ind += 1;
            // Should not clone here but I am not ready to fight with borrow checker yet.
            self.top_line = self.current_line.clone();
            self.current_line = self.bottom_line.clone();

            // We are going to point at the last line.
            if self.current_row_ind == self.height() - 1 {
                // The last virual line.
                self.bottom_line = vec!['.'; self.width()];
            } else {
                self.bottom_line = self.input[self.current_row_ind + 1].chars().collect();
            }

            true
        }

        pub fn sum_current_line(&mut self) -> i32 {
            assert!(self.current_row_ind < self.input.len());

            let mut sum = 0;
            let mut i = 0;
            while i < self.current_line.len() {
                if let Some((next_i, number)) = self.find_next_number(i) {
                    i = next_i;
                    sum += number;
                } else {
                    break;
                }
            }

            sum
        }

        fn find_next_number(&self, start_pos: usize) -> Option<(usize, i32)> {
            // start_i is the first char in a number sequence.
            let mut i = start_pos;
            loop {
                if let Some(num_start_i) = self.skip(i) {
                    i = num_start_i;
                    // num_end_i is the next char after the number sequence.
                    if let Some((num_end_i, num)) = self.read_num(i) {
                        let adjacents = self.look_around(num_start_i, num_end_i);
                        for adj in adjacents {
                            if !self.skip_characters.contains(&adj) {
                                return Some((num_end_i, num));
                            }
                        }
                        i = num_end_i;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            return None;
        }

        fn skip(&self, start_pos: usize) -> Option<usize> {
            let mut i = start_pos;
            while i < self.width() && !self.current_line[i].is_numeric() {
                i += 1;
            }

            if i == self.width() {
                None
            } else {
                Some(i)
            }
        }

        fn read_num_bidirect(&self, point_position: usize, line: &Vec<char>) -> i32 {
            let mut i = point_position as i32;
            while i > -1 && line[i as usize].is_numeric() {
                i -= 1;
            }

            return self.read_num_from(line, (i + 1) as usize).unwrap().1;
        }

        fn look_around_gear_for_numbers(&self, i: usize) -> Vec<i32> {
            let mut result = Vec::new();

            // process top line
            // it means two numbers are possible to be connected on top
            if !self.top_line[i].is_numeric() {
                // top left corner
                if i != 0 && self.top_line[i - 1].is_numeric() {
                    result.push(self.read_num_bidirect(i - 1, &self.top_line));
                }

                // top right corner
                if i != (self.width() - 1) && self.top_line[i + 1].is_numeric() {
                    result.push(self.read_num_bidirect(i + 1, &self.top_line));
                }
            } else {
                // only one number is posible on top
                result.push(self.read_num_bidirect(i, &self.top_line));
            }

            // process bottom line
            // it means two numbers are possible to be connected in bottom
            if !self.bottom_line[i].is_numeric() {
                // bottom left corner
                if i != 0 && self.bottom_line[i - 1].is_numeric() {
                    result.push(self.read_num_bidirect(i - 1, &self.bottom_line));
                }

                // bottom right corner
                if i != (self.width() - 1) && self.bottom_line[i + 1].is_numeric() {
                    result.push(self.read_num_bidirect(i + 1, &self.bottom_line));
                }
            } else {
                // only one number is posible in bottom
                result.push(self.read_num_bidirect(i, &self.bottom_line));
            }

            // process left and right current line
            if i != 0 && self.current_line[i - 1].is_numeric() {
                result.push(self.read_num_bidirect(i - 1, &self.current_line));
            }

            if i != (self.width() - 1) && self.current_line[i + 1].is_numeric() {
                result.push(self.read_num_bidirect(i + 1, &self.current_line));
            }

            return result;
        }

        fn read_num_from(&self, line: &Vec<char>, start_pos: usize) -> Option<(usize, i32)> {
            let mut i = start_pos;
            let mut num_str = String::new();

            while i < line.len() && line[i].is_numeric() {
                num_str.push(line[i]);
                i += 1;
            }

            if i == line.len() && !line[i - 1].is_numeric() {
                None
            } else {
                Some((i, num_str.parse().unwrap()))
            }
        }

        fn read_num(&self, start_pos: usize) -> Option<(usize, i32)> {
            self.read_num_from(&self.current_line, start_pos)
        }

        // .....
        // .321# <-- curent line
        // ...*.
        //
        // look_around(1, 4) will return
        // ['.','.', '.', '.', '.', '.', '#', '.', '.', '.', '*', '.']
        fn look_around(&self, start_pos: usize, end_pos: usize) -> Vec<char> {
            assert!(self.bottom_line.len() == self.current_line.len());
            assert!(self.current_line.len() == self.top_line.len());

            let mut result = Vec::new();
            let mut i = start_pos;

            if i != 0 {
                result.push(self.top_line[i - 1]);
                result.push(self.current_line[i - 1]);
                result.push(self.bottom_line[i - 1]);
            }

            while i < end_pos {
                result.push(self.top_line[i]);
                result.push(self.bottom_line[i]);
                i += 1;
            }

            if i != self.width() {
                result.push(self.top_line[i]);
                result.push(self.current_line[i]);
                result.push(self.bottom_line[i]);
            }

            result
        }

        fn find_next_gear_numbers(&self, start_pos: usize) -> Option<(usize, i32, i32)> {
            let mut i = start_pos;
            while i < self.width() && self.current_line[i] != '*' {
                i += 1;
            }

            if i == self.width() {
                return None;
            }

            let numbers = self.look_around_gear_for_numbers(i);
            if numbers.len() == 2 {
                return Some((i + 1, numbers[0], numbers[1]));
            } else {
                return Some((i + 1, 0, 0));
            }
        }

        pub fn sum_current_line_gear(&mut self) -> i64 {
            assert!(self.current_row_ind < self.input.len());

            let mut sum = 0;
            let mut i = 0;
            while i < self.current_line.len() {
                if let Some((next_i, number1, number2)) = self.find_next_gear_numbers(i) {
                    i = next_i;
                    sum += number1 as i64 * number2 as i64;
                } else {
                    break;
                }
            }

            sum
        }
    }
}

pub mod solution1 {
    use crate::common;

    pub fn process(input: &Vec<String>) -> i32 {
        let mut total_sum = 0;
        let mut cursor = common::Cursor::init(&input);

        loop {
            let s = cursor.sum_current_line();
            total_sum += s;
            if !cursor.move_next() {
                break;
            }
        }

        total_sum
    }
}

pub mod solution2 {
    use crate::common;

    pub fn process(input: &Vec<String>) -> i64 {
        let mut total_sum = 0;
        let mut cursor = common::Cursor::init(&input);

        loop {
            let s = cursor.sum_current_line_gear();
            total_sum += s;
            if !cursor.move_next() {
                break;
            }
        }

        total_sum
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn exmaple_2_works() {
        let input = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        assert_eq!(467 * 35 + 755 * 598, solution2::process(&input));
    }

    #[test]
    fn corner_cases() {
        let input = vec![
            "11.....", "11*....", ".......", "22*22..", "......3", ".....*3", ".......", "....10*",
            ".....10", "..44...", "...*...", "....44.", "*55....", ".55..66", "......*", ".....66",
            "77.77..", "..*...."
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        assert_eq!(
            11 * 11 + 22 * 22 + 3 * 3 + 10 * 10 + 44 * 44 + 55 * 55 + 66 * 66 + 77 * 77,
            solution2::process(&input)
        );
    }

    #[test]
    fn real_input() {
        let line1_sum = 0;
        let line2_sum = 860 * 985;
        let line3_sum = 0;

        let input = vec![
"........518..........918..-....472..172....776......207............38........................860..............274..945.....162..............",
"....984.....%...............+..712...83..*....130..................+....*...545.............*......+.............../.727./....826......*....",
"................490......519../...........16....%...42.822..486......214..../...............985.480..............798....................249.",
        ]
            .iter()
            .map(|&s| s.into())
            .collect();

        assert_eq!(
            line1_sum + line2_sum + line3_sum,
            solution2::process(&input)
        )
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        assert_eq!(4361, solution1::process(&input))
    }

    #[test]
    fn corner_cases() {
        let input = vec![
            "11.11.11", // 22
            "..1.....", // 1
            "........", // 0
            "........", // 0
            "*11..-11", // 22
            "........", // 0
            ".11....1", // 12
            "*.....*.", // 0
            ".....*..", // 0
            "1*....11", // 12
            "........", // 0
            ".*...*11", // 11
            "1.......", // 1
            "......11", // 11
            "1....*..", // 1
            ".*......", // 0
            "*.....11", // 11
            ".1..*.*.", // 1
            "....1..*", // 1
            "......1.", // 1
            "....1...", // 1
            "....*.1.", // 1
            ".......*", // 0
            "........", // 0
            ".1*1*1*1", // 4
            "........", // 0
            ".11*....", // 11
            "....11..", // 11
            "........", // 0
            "....11..", // 11
            ".11*....", // 11
            "......1*", // 1
            "*1.1*...", // 2
            "........", // 0
            "...1....", // 0
            ".....11.", // 0
            "........", // 0
            ".1.1.1.1", // 4
            "1.1.1.1.", // 4
            "........", // 0
            ".2*2..*.", // 4
            "..2...11", // 13
            "........", // 0
            "....22..", // 22
            "..22..22", // 44
            "........", // 0
            "......11", // 11
            ".......*", // 0
            "......11", // 11
            "....1...", // 0
            "..1...1.", // 0
            "....1...", // 0
            "........", // 0
            "*..*...1", // 1
            ".1.1.1*.", // 3
            "..1.1...", // 2
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        assert_eq!(279, solution1::process(&input));
    }

    #[test]
    fn a_lot_digits() {
        let input = vec!["900", "*90", "90*", "900", "9*9"]
            .iter()
            .map(|&s| s.into())
            .collect();

        assert_eq!(1998, solution1::process(&input))
    }

    #[test]
    fn only_single_digit() {
        let input = vec!["...", ".9.", "*.."]
            .iter()
            .map(|&s| s.into())
            .collect();

        assert_eq!(9, solution1::process(&input))
    }

    #[test]
    fn first_lines_from_real_input() {
        let first_line_sum = 452 + 712 + 996 + 646 + 40 + 1 + 958 + 553;
        let second_line_sum = 661 + 844 + 781 + 163 + 698 + 239 + 57;
        let third_line_sum = 139 + 282 + 301;
        let forth_line_sum = 918 + 172 + 776 + 860;

        let input = vec![
".........398.............551.....................452..................712.996.................646.40...1.....875..958.553...................",
"..................................661..-844......*.../781...835..#163....*.......698.239.........*.....*.............*............*57.......",
".....................&...............*......+..139..................................*.........-.......282......................301..........",
"........518..........918..-....472..172....776......207............38........................860..............274..945.....162.............."
        ]
            .iter()
            .map(|&s| s.into())
            .collect();

        assert_eq!(
            first_line_sum + second_line_sum + third_line_sum + forth_line_sum,
            solution1::process(&input)
        )
    }
}
