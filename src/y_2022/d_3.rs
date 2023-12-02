use std::collections::HashMap;

use crate::prelude::*;

pub struct Day3 {
    inp: Vec<Vec<u8>>,
}

impl Questions for Day3 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.inp = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|v| v.chars().into_iter().map(|j| j as u8).collect::<Vec<u8>>())
            .collect::<Vec<_>>();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;

        for v in self.inp.clone().into_iter() {
            let mut map = HashMap::new();
            let mid_point = v.len() / 2;

            for (i, c) in v.into_iter().enumerate() {
                if i > mid_point - 1 {
                    let does_contain = map.contains_key(&c);

                    if does_contain {
                        sum += Day3::fetch_value(c);

                        break;
                    }
                } else {
                    map.insert(c, 1);
                }
            }
        }

        let ans = sum.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;

        let mut line_num = 0;
        let mut map = HashMap::new();

        for v in self.inp.clone().into_iter() {
            if line_num % 3 == 0 {
                line_num = 0;
                map = HashMap::new();
            }

            let mut tmp = vec![];

            for c in v.into_iter() {
                if !tmp.contains(&c) {
                    let v = match map.get(&c) {
                        Some(v) => *v,
                        None => 0,
                    };

                    // this is line 3 since we start the line numbers from 0
                    if line_num == 2 {
                        if v == 2 {
                            sum += Day3::fetch_value(c);

                            break;
                        }
                    }

                    map.insert(c, v + 1);

                    tmp.push(c);
                }
            }

            line_num += 1;
        }

        let ans = sum.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 { inp: vec![] }
    }

    pub fn fetch_value(mut c: u8) -> u8 {
        let val = c as char;
        if val.is_lowercase() {
            c = c - 96;
        } else {
            c = (c - 65) + 27;
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("157");

        let mut day3 = Day3::new();

        day3.init("inputs/2022/3a.txt")
            .expect("error trying to init day3");

        let q1 = day3.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("70");

        let mut day3 = Day3::new();

        day3.init("inputs/2022/3a.txt")
            .expect("error trying to init day3");

        let q2 = day3.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
