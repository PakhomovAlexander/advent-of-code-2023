pub mod common {
    #[derive(Debug, PartialEq)]
    pub struct Bubles {
        pub r: usize,
        pub g: usize,
        pub b: usize,
    }

    impl Bubles {
        pub fn new(r: usize, g: usize, b: usize) -> Bubles {
            return Bubles { r, g, b };
        }

        pub fn pow(&self) -> usize {
            let mut p = 1;
            if self.r != 0 { p *= self.r }
            if self.g != 0 { p *= self.g }
            if self.b != 0 { p *= self.b }

            return p;
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Game {
        pub id: usize,
        pub rounds: Vec<Bubles>,
    }

    impl Game {
        pub fn find_minimum_possible_bubles(&self) -> Bubles {
            let mut bubles = Bubles::new(0,0,0);
            for b in &self.rounds {
                if b.r > bubles.r {
                    bubles.r = b.r;
                }
                if b.g > bubles.g {
                    bubles.g = b.g;
                }
                if b.b > bubles.b {
                    bubles.b = b.b;
                }
            }

            return bubles;
        }

        pub fn possible(&self, game: &Game) -> bool {
            for bubles in &game.rounds {
                if self.rounds[0].r < bubles.r
                    || self.rounds[0].g < bubles.g
                    || self.rounds[0].b < bubles.b
                {
                    return false;
                }
            }

            return true;
        }

        pub fn lookup_buble(tokens: &Vec<String>, start_pos: usize) -> (Bubles, usize) {
            let mut bubles = Bubles::new(0, 0, 0);
            let mut i = start_pos;

            while i + 1 < tokens.len() {
                let (first, second) = (&tokens[i], &tokens[i + 1]);
                match first.parse::<usize>() {
                    Ok(num) => {
                        match second.as_ref() {
                            "blue" => bubles.b += num,
                            "red" => bubles.r += num,
                            "green" => bubles.g += num,
                            _ => panic!("Illegal state"),
                        }

                        i += 2;
                    }
                    Err(_) => {
                        if first == "," {
                            i += 1;
                            continue;
                        } else if first == ";" {
                            return (bubles, i);
                        }
                    }
                }
            }

            return (bubles, i);
        }

        // Parse the line, example:
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        pub fn parse(line: &str) -> Game {
            let tokens = tokens(line);
            assert!(tokens[0] == "Game");
            let id: usize = tokens[1].parse().unwrap();

            let mut rounds: Vec<Bubles> = Vec::new();

            let mut i = 2;
            while i < tokens.len() {
                if tokens[i] == ";" || tokens[i] == ":" {
                    let (bubles, ind) = Self::lookup_buble(&tokens, i + 1);
                    rounds.push(bubles);
                    i = ind;
                } else {
                    panic!("Unexpected input");
                }
            }

            return Game { id, rounds };
        }

        pub fn new(id: usize, rounds: Vec<Bubles>) -> Game {
            return Game { id, rounds };
        }
    }

    pub fn tokens(s: &str) -> Vec<String> {
        let chars: Vec<char> = s.chars().collect();
        let mut tokens: Vec<String> = Vec::new();
        let mut current_token = String::new();

        for ch in chars {
            match ch {
                ' ' => {
                    if current_token.len() > 0 {
                        tokens.push(current_token.clone());
                        current_token = String::new();
                    }
                }
                ':' | ',' | ';' => {
                    if current_token.len() > 0 {
                        tokens.push(current_token);
                        current_token = String::new();
                    }
                    tokens.push(ch.to_string());
                }
                _ => current_token.push(ch),
            }
        }

        if current_token.len() > 0 {
            tokens.push(current_token);
        }

        return tokens;
    }
}

pub mod solution1 {
    use crate::common::{Game, Bubles};

    pub fn sum_of_possible_games_ids(input: &Vec<String>) -> usize {
        let setup = Game::new(0, vec![Bubles::new(12, 13, 14)]);
        let mut sum = 0;
        for line in input {
            let g = Game::parse(line);
            if setup.possible(&g) {
                sum += g.id;
            }
        }

        return sum;
    }
}

pub mod solution2 {
    use crate::common::Game;

    pub fn sum_of_powers(input: &Vec<String>) -> usize {
        let mut sum = 0;
        for line in input {
            let g = Game::parse(line);
            let bubles = g.find_minimum_possible_bubles();
            sum += bubles.pow();
        }

        return sum;
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;
    use common::Bubles;
    use common::Game;

    #[test]
    fn tokenizer() {
        let tokens = common::tokens("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(
            tokens,
            vec![
                "Game", "1", ":", "3", "blue", ",", "4", "red", ";", "1", "red", ",", "2", "green",
                ",", "6", "blue", ";", "2", "green"
            ]
        );
    }

    #[test]
    fn parse_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected_game = Game::new(
            1,
            vec![
                Bubles::new(4, 0, 3),
                Bubles::new(1, 2, 6),
                Bubles::new(0, 2, 0),
            ],
        );

        assert_eq!(Game::parse(line), expected_game);
    }

    #[test]
    fn example_work() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        let sum = solution1::sum_of_possible_games_ids(&input);
        assert_eq!(8, sum);
    }
}


#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn example_work() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();

        let sum = solution2::sum_of_powers(&input);
        assert_eq!(2286, sum);
    }

}
