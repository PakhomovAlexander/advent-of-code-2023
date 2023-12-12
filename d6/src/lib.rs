pub mod common {
    pub fn read_vec(line: &String) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            while !chars[i].is_numeric() {
                i += 1;
            }
            let mut curr_num = String::new();
            while i < chars.len() && chars[i].is_numeric() {
                curr_num.push(chars[i]);
                i += 1;
            }
            res.push(curr_num.parse().unwrap());
            i += 1;
        }

        res
    }

    pub fn read_num(line: &String) -> i64 {
        let chars: Vec<char> = line.chars().collect();
        let mut curr_num = String::new();
        let mut i = 0;
        while i < chars.len() {
            while !chars[i].is_numeric() {
                i += 1;
            }
            while i < chars.len() && chars[i].is_numeric() {
                curr_num.push(chars[i]);
                i += 1;
            }
            i += 1;
        }

        curr_num.parse().unwrap()
    }
}

pub mod solution1 {
    use crate::common;

    fn get_distance(hold_time: i32, total_time: i32) -> i32 {
        let movement_time = total_time - hold_time;
        let speed = hold_time;

        movement_time * speed
    }

    fn get_number_of_variants(t: i32, d: i32) -> i32 {
        let mut cnt = 0;
        for i in 0..t + 1 {
            if get_distance(i, t) > d {
                cnt += 1;
            }
        }

        cnt
    }

    pub fn process(input: &Vec<String>) -> i64 {
        let time = common::read_vec(&input[0]);
        let distance = common::read_vec(&input[1]);

        let mut res = 1;
        let mut i = 0;
        while i < time.len() {
            let t = time[i];
            let d = distance[i];
            let number_of_variants = get_number_of_variants(t, d);
            if number_of_variants > 0 {
                res *= number_of_variants as i64;
            }
            i += 1;
        }

        res
    }
}

pub mod solution2 {
    use crate::common;

    fn get_distance(hold_time: i32, total_time: i32) -> i64 {
        let movement_time = total_time as i64 - hold_time as i64;
        let speed = hold_time as i64;

        movement_time * speed
    }

    fn get_number_of_variants(t: i32, d: i64) -> i32 {
        let mut cnt = 0;
        for i in 0..t + 1 {
            if get_distance(i, t) > d {
                cnt += 1;
            }
        }

        cnt
    }

    pub fn process(input: &Vec<String>) -> i32 {
        let time = common::read_num(&input[0]);
        let distance = common::read_num(&input[1]);

        get_number_of_variants(time as i32, distance)
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn example_works() {
        let input = vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ];

        assert_eq!(288, solution1::process(&input));
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn example_works() {
        let input = vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ];

        assert_eq!(71503, solution2::process(&input));
    }
}
