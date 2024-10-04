use std::{collections::HashMap, i32};

use crate::prelude::*;

pub struct Day6 {
    data: Vec<Vec<u8>>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2016/6.txt";
// const TEST_FILE_NAME: &str = "inputs/2016/6a.txt";

impl Questions for Day6 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.data = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|w| w.trim().chars().map(|c| c as u8).collect())
            .collect();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut frequencies = Vec::with_capacity(self.data[0].len());

        for _ in 0..self.data[0].len() {
            frequencies.push(HashMap::new());
        }

        for word in &self.data {
            for (idx, ch) in word.iter().enumerate() {
                let count = match frequencies[idx].get(&ch) {
                    None => 1,
                    Some(n) => n + 1,
                };

                frequencies[idx].insert(ch, count);
            }
        }

        let mut ans = vec![];

        for map in frequencies {
            let mut highestest_count = i32::MIN;
            let mut highest_char = 0;

            for (key, value) in map {
                if value > highestest_count {
                    highestest_count = value;
                    highest_char = *key;
                }
            }

            ans.push(highest_char);
        }

        let ans = ans
            .iter()
            .map(|n| (*n as char).to_string())
            .collect::<Vec<String>>()
            .join("");

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut frequencies = Vec::with_capacity(self.data[0].len());

        for _ in 0..self.data[0].len() {
            frequencies.push(HashMap::new());
        }

        for word in &self.data {
            for (idx, ch) in word.iter().enumerate() {
                let count = match frequencies[idx].get(&ch) {
                    None => 1,
                    Some(n) => n + 1,
                };

                frequencies[idx].insert(ch, count);
            }
        }

        let mut ans = vec![];

        for map in frequencies {
            let mut least_count = i32::MAX;
            let mut least_char = 0;

            for (key, value) in map {
                if value < least_count {
                    least_count = value;
                    least_char = *key;
                }
            }

            ans.push(least_char);
        }

        let ans = ans
            .iter()
            .map(|n| (*n as char).to_string())
            .collect::<Vec<String>>()
            .join("");

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 { data: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("easter");

        let mut day6 = Day6::new();

        day6.init("inputs/2016/6a.txt")
            .expect("error trying to init day6");

        let q1 = day6.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("advent");

        let mut day6 = Day6::new();

        day6.init("inputs/2016/6a.txt")
            .expect("error trying to init day6");

        let q2 = day6.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
