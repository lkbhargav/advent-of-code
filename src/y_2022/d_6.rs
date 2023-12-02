use std::collections::HashSet;

use crate::prelude::*;

pub struct Day6 {
    inp: Vec<char>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2022/6.txt";

impl Questions for Day6 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.inp = contents.chars().collect::<Vec<char>>();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut set = HashSet::new();
        let mut ans = 0;

        for (idx, _) in self.inp.iter().enumerate() {
            if !(idx + 4 < self.inp.len()) {
                break;
            }

            for i in 0..=3 {
                set.insert(self.inp[idx + i]);
            }

            if set.len() == 4 {
                ans = idx + 3 + 1; // +1 at the end because we want the character position not the index itself
                break;
            }

            set.clear();
        }

        let ans = ans.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut set = HashSet::new();
        let mut ans = 0;

        for (idx, _) in self.inp.iter().enumerate() {
            if !(idx + 14 < self.inp.len()) {
                break;
            }

            for i in 0..=13 {
                set.insert(self.inp[idx + i]);
            }

            if set.len() == 14 {
                ans = idx + 13 + 1; // +1 at the end because we want the character position not the index itself
                break;
            }

            set.clear();
        }

        let ans = ans.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 { inp: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("11");

        let mut day6 = Day6::new();

        day6.init("inputs/2022/6a.txt")
            .expect("error trying to init day6");

        let q1 = day6.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("26");

        let mut day6 = Day6::new();

        day6.init("inputs/2022/6a.txt")
            .expect("error trying to init day6");

        let q2 = day6.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
