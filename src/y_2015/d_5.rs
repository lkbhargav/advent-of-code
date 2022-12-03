use crate::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum State {
    Valid,
    Invalid,
    Unknown,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NiceOrNot {
    rule_1: State,
    rule_2: State,
    rule_3: State,
    vowels_count: u8,
    valid: State,
    word: Vec<u8>,
}

impl NiceOrNot {
    pub fn new(word: &str) -> Self {
        let word = word.as_bytes().to_vec();

        NiceOrNot {
            rule_1: State::Unknown,
            rule_2: State::Unknown,
            rule_3: State::Unknown,
            vowels_count: 0,
            valid: State::Unknown,
            word,
        }
    }

    pub fn check_rule_1(&mut self, v: u8) {
        if self.rule_1 == State::Unknown && self.valid == State::Unknown {
            match v {
                97 | 101 | 105 | 111 | 117 => {
                    self.vowels_count += 1;

                    if self.vowels_count == 3 {
                        self.rule_1 = State::Valid;
                        self.validate();
                    }
                }
                _ => (),
            }
        }
    }

    pub fn check_rule_2(&mut self, a: u8, b: u8) {
        if self.rule_2 == State::Unknown && self.valid == State::Unknown {
            if a == b {
                self.rule_2 = State::Valid;
                self.validate();
            }
        }
    }

    pub fn check_rule_3(&mut self, a: u8, b: u8) {
        if self.rule_3 == State::Unknown && self.valid == State::Unknown {
            if a == b - 1 && vec![98, 100, 113, 121].contains(&(b as i32)) {
                self.rule_3 = State::Invalid;
                self.validate();
            }
        }
    }

    /// Needs to be run at the end for accuracy
    pub fn wrap_up(&mut self) {
        if self.rule_3 == State::Unknown {
            self.rule_3 = State::Valid;
            self.validate();
        }
    }

    pub fn validate(&mut self) {
        if self.rule_1 == State::Valid && self.rule_2 == State::Valid && self.rule_3 == State::Valid
        {
            self.valid = State::Valid;
        } else if self.rule_1 == State::Invalid
            || self.rule_2 == State::Invalid
            || self.rule_3 == State::Invalid
        {
            self.valid = State::Invalid;
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct NiceOrNotV2 {
    rule_1: State,
    rule_2: State,
    rule_1_map: HashMap<(u8, u8), u8>,
    valid: State,
    word: Vec<u8>,
}

impl NiceOrNotV2 {
    pub fn new(word: &str) -> NiceOrNotV2 {
        let word = word.as_bytes().to_vec();

        NiceOrNotV2 {
            rule_1: State::Unknown,
            rule_1_map: HashMap::new(),
            rule_2: State::Unknown,
            valid: State::Unknown,
            word,
        }
    }

    pub fn check_rule_1(&mut self, prev: Option<u8>, pair: (u8, u8), prev_pair: bool) -> bool {
        // invalid condition. checks for `aaa` like sequences
        if prev_pair && prev.is_some() && prev.unwrap() == pair.0 && prev.unwrap() == pair.1 {
            return false;
        }

        let val = match self.rule_1_map.get(&pair) {
            Some(v) => *v,
            None => 0,
        };

        self.rule_1_map.insert(pair, val + 1);

        true
    }

    pub fn check_rule_2(&mut self, triplet: (u8, u8, u8)) {
        if self.rule_2 == State::Unknown && triplet.0 == triplet.2 {
            self.rule_2 = State::Valid;
            self.validate();
        }
    }

    pub fn wrap_up(&mut self) {
        // handling rule 1
        for (_, v) in &self.rule_1_map {
            if *v >= 2 {
                self.rule_1 = State::Valid;
            }
        }

        if self.rule_1 == State::Unknown {
            self.rule_1 = State::Invalid;
        }

        if self.rule_2 == State::Unknown {
            self.rule_2 = State::Invalid;
        }

        self.validate();
    }

    pub fn validate(&mut self) {
        if self.rule_1 == State::Valid && self.rule_2 == State::Valid {
            self.valid = State::Valid;
        } else if self.rule_1 == State::Invalid || self.rule_2 == State::Invalid {
            self.valid = State::Invalid;
        }
    }
}

pub struct Day5 {
    q1: Vec<NiceOrNot>,
    q2: Vec<NiceOrNotV2>,
}

impl Questions for Day5 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.q1 = contents
            .lines()
            .filter(|v| !v.is_empty())
            .map(|v| {
                let v = v.trim();
                NiceOrNot::new(v)
            })
            .collect();

        self.q2 = contents
            .lines()
            .filter(|v| !v.is_empty())
            .map(|v| {
                let v = v.trim();
                NiceOrNotV2::new(v)
            })
            .collect();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let results = self
            .q1
            .clone()
            .into_iter()
            .map(|mut v| {
                v.word.clone().into_iter().enumerate().for_each(|(i, u)| {
                    v.check_rule_1(u);

                    if i > 0 {
                        v.check_rule_2(v.word[i - 1], u);
                        v.check_rule_3(v.word[i - 1], u);
                    }
                });

                v.wrap_up();

                v.valid
            })
            .filter(|v| *v == State::Valid)
            .collect::<Vec<State>>();

        let ans = results.len().to_string();

        println!("\nAnswer to first question is {}!\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut prev_pair = false;
        let results = self
            .q2
            .clone()
            .into_iter()
            .map(|mut v| {
                v.word.clone().into_iter().enumerate().for_each(|(i, u)| {
                    if i + 1 < v.word.len() {
                        prev_pair = v.check_rule_1(
                            if i == 0 { None } else { Some(v.word[i - 1]) },
                            (u, v.word[i + 1]),
                            prev_pair,
                        );
                    }

                    if i + 2 < v.word.len() {
                        v.check_rule_2((u, v.word[i + 1], v.word[i + 2]));
                    }
                });

                v.wrap_up();

                v.valid
            })
            .filter(|v| *v == State::Valid)
            .collect::<Vec<State>>();

        let ans = results.len().to_string();

        println!("\nAnswer to second question is {}!\n", ans.green());

        Ok(ans)
    }
}

impl Day5 {
    pub fn new() -> Day5 {
        Day5 {
            q1: vec![],
            q2: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("2");

        let mut day5 = Day5::new();

        day5.init("inputs/2015/5a.txt")
            .expect("error trying to init day5");

        let q1 = day5.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("3");

        let mut day5 = Day5::new();

        day5.init("inputs/2015/5b.txt")
            .expect("error trying to init day5");

        let q2 = day5.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
