use std::collections::HashSet;

use crate::prelude::*;

pub struct Day1 {
    data: Vec<i32>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2018/1.txt";
// const TEST_FILE_NAME: &str = "inputs/2018/1a.txt";

impl Questions for Day1 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let data = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|l| l.trim())
            .map(|l| l.parse::<i32>().expect("trying to parse str to i32"))
            .collect::<Vec<i32>>();

        self.data = data;

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut total = 0;

        for d in &self.data {
            total += d;
        }

        let ans = total.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut set = HashSet::new();
        let found_freq;
        let mut freq = 0;

        'outer: loop {
            for d in &self.data {
                freq += d;

                if set.contains(&freq) {
                    found_freq = freq;
                    break 'outer;
                }

                set.insert(freq);
            }
        }

        let ans = found_freq.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 { data: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("3");

        let mut day1 = Day1::new();

        day1.init("inputs/2018/1a.txt")
            .expect("error trying to init day1");

        let q1 = day1.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("2");

        let mut day1 = Day1::new();

        day1.init("inputs/2018/1a.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
