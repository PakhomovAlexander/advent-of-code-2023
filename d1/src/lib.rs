pub mod solution {
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

    pub fn sum_of_colibration_values(input: Vec<String>) -> i32 {
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
mod tests {
    use super::*;

    #[test]
    fn example_works() {
        let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"].iter().map(|&s| s.into()).collect();

        let sum = solution::sum_of_colibration_values(input);

        assert_eq!(sum, 142);
    }
}
