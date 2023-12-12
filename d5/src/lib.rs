pub mod common {

    #[derive(Debug)]
    pub struct Part {
        pub destination_start: i64,
        pub source_start: i64,
        pub size: i64,
    }

    impl Part {
        pub fn parse(line: &String) -> Part {
            let nums = numbers(line);
            if nums.len() != 3 {
                panic!("Expected 3 nums in a map line");
            }

            Part {
                destination_start: nums[0],
                source_start: nums[1],
                size: nums[2],
            }
        }
    }

    #[derive(Debug)]
    pub struct Map {
        pub parts: Vec<Part>,
    }

    impl Map {
        pub fn sort(&mut self) {
            self.parts
                .sort_by(|a, b| a.destination_start.cmp(&b.destination_start));
        }

        pub fn convert(&self, value: i64) -> i64 {
            for part in &self.parts {
                // Check if current part can convert given value
                if value >= part.source_start && value <= part.source_start + part.size {
                    let diff = value - part.source_start;
                    return part.destination_start + diff;
                }
            }

            return value;
        }

        // given [start, end] returns Vec of [start, end] that leads to original [start, end]
        // result is sorted in a way that first element leads to the minimum element of
        // the original pair
        pub fn find_paths(&self, start: i64, end: i64) -> Vec<(i64, i64)> {
            let mut res = Vec::new();
            for p in &self.parts {
                if let Some((l, r)) = Self::intersection(
                    p.destination_start,
                    p.destination_start + p.size,
                    start,
                    end,
                ) {
                    let l_diff = l - p.destination_start;
                    let r_diff = r - p.destination_start;
                    res.push((p.source_start + l_diff, p.source_start + r_diff));
                }
            }

            return res;
        }

        pub fn intersection(l1: i64, r1: i64, l2: i64, r2: i64) -> Option<(i64, i64)> {
            // l1  l2  r1
            if l1 >= l2 && l2 <= r1 {
                let l_inter = l2;
                let r_inter = if r2 <= r1 { r2 } else { r1 };
                return Some((l_inter, r_inter));
            }

            // l2 l1 r2
            if l2 >= l1 && l1 <= r2 {
                let l_inter = l1;
                let r_inter = if r1 <= r2 { r1 } else { r2 };
                return Some((l_inter, r_inter));
            }

            return None;
        }
    }

    pub fn numbers(line: &String) -> Vec<i64> {
        let mut result: Vec<i64> = Vec::new();
        let chars: Vec<char> = line.chars().collect();

        let mut i = 0;
        while i < chars.len() {
            let mut curr_num = String::new();
            while i < chars.len() && chars[i].is_numeric() {
                curr_num.push(chars[i]);
                i += 1;
            }

            if curr_num.len() != 0 {
                result.push(curr_num.parse().unwrap());
            }

            if i < chars.len() && chars[i] == ' ' {
                i += 1;
            }
        }

        result
    }
}

pub mod solution1 {
    use crate::common;

    pub fn process(input: &Vec<String>) -> i64 {
        let mut i = 0;
        let mut seeds = common::numbers(&String::from(&input[i][6..])); // skip 'seeds: '
        i += 2;

        while i < input.len() {
            let mut current_map = common::Map { parts: vec![] };
            i += 1; // i at the 'map-name map:' line, skip it
            while i < input.len() && input[i].len() != 0 {
                let part = common::Part::parse(&input[i]);
                current_map.parts.push(part);
                i += 1;
            }

            for seed in &mut seeds {
                *seed = current_map.convert(*seed);
            }

            i += 1; // skip an empty line
        }

        let mut min = seeds[0];
        for seed in &seeds {
            if min > *seed {
                min = *seed;
            }
        }

        min
    }
}

pub mod solution2 {
    use crate::common;

    fn read_seeds(line: &String) -> Vec<(i64, i64)> {
        let seeds_definition = common::numbers(&String::from(&line[6..])); // skip 'seeds: '
        let mut seeds = Vec::new();

        let mut i = 0;
        while i < seeds_definition.len() - 1 {
            let seed_start = seeds_definition[i];
            let cnt = seeds_definition[i + 1];
            seeds.push((seed_start, cnt));

            i += 2;
        }

        seeds
    }

    pub fn process(input: &Vec<String>) -> i64 {
        let mut i = 2;
        let seeds = read_seeds(&input[0]);

        let mut maps = Vec::new();

        while i < input.len() {
            let mut current_map = common::Map { parts: vec![] };
            i += 1; // i at the 'map-name map:' line, skip it
            while i < input.len() && input[i].len() != 0 {
                let part = common::Part::parse(&input[i]);
                current_map.parts.push(part);
                i += 1;
            }
            current_map.sort();
            maps.push(current_map);

            i += 1; // skip an empty line
        }

        let mut min = 100000000;
        for (start_seed, cnt) in seeds {
            println!("Start processing seeds: ({}, {})", start_seed, cnt);
            let upper_seed = start_seed;
            let mut curr_diff = 0;
            while curr_diff < cnt {
                let mut running_seed = upper_seed + curr_diff;
                for map in &maps {
                    running_seed = map.convert(running_seed);
                }

                if running_seed < min {
                    min = running_seed;
                }
                curr_diff += 1;
            }

        }

        return min;
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn example_works() {
        let input = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4 ",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        assert_eq!(46, solution2::process(&input))
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn example_works() {
        let input = vec![
            "seeds: 79 14 55 13",
            "",
            "seed-to-soil map:",
            "50 98 2",
            "52 50 48",
            "",
            "soil-to-fertilizer map:",
            "0 15 37",
            "37 52 2",
            "39 0 15",
            "",
            "fertilizer-to-water map:",
            "49 53 8",
            "0 11 42",
            "42 0 7",
            "57 7 4",
            "",
            "water-to-light map:",
            "88 18 7",
            "18 25 70",
            "",
            "light-to-temperature map:",
            "45 77 23",
            "81 45 19",
            "68 64 13",
            "",
            "temperature-to-humidity map:",
            "0 69 1",
            "1 0 69",
            "",
            "humidity-to-location map:",
            "60 56 37",
            "56 93 4 ",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        assert_eq!(35, solution1::process(&input))
    }
}
