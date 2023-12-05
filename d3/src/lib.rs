pub mod solution1 {
    #[derive(Debug)]
    struct Cursor<'a> {
        input: &'a Vec<String>,
        top_line: Vec<char>,
        current_line: Vec<char>,
        bottom_line: Vec<char>,
        current_row_ind: usize,
        skip_characters: Vec<char>,
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

        fn read_num(&self, start_pos: usize) -> Option<(usize, i32)> {
            let mut i = start_pos;
            let mut num_str = String::new();

            while i < self.width() && self.current_line[i].is_numeric() {
                num_str.push(self.current_line[i]);
                i += 1;
            }

            if i == self.width() && !self.current_line[i - 1].is_numeric() {
                None
            } else {
                Some((i, num_str.parse().unwrap()))
            }
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
    }

    pub fn process(input: &Vec<String>) -> i32 {
        let mut total_sum = 0;
        let mut cursor = Cursor::init(&input);

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
        dbg!(first_line_sum);
        let second_line_sum = 661 + 844 + 781 + 163 + 698 + 239 + 57;
        dbg!(second_line_sum);
        let third_line_sum = 139 + 282 + 301;
        dbg!(third_line_sum);
        let forth_line_sum = 918 + 172 + 776 + 860;
        dbg!(forth_line_sum);

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
