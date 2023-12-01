pub mod adv_io {
    use std::fs::read_to_string;

    pub fn read_input(path: &str) -> Vec<String> {
        let mut result = Vec::new();

        for line in read_to_string(path).unwrap().lines() {
            result.push(line.to_string())
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use adv_io;

    #[test]
    fn it_works() {
        assert_eq!(
            adv_io::read_input(&"testdata/input"),
            vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
        );
    }
}
