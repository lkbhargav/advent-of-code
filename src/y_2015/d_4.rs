use crate::prelude::*;
use md5::{self, Digest};
use std::str;

pub struct Day4 {
    inp: String,
}

impl Questions for Day4 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.inp = contents.trim().to_string();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut step = 0;

        loop {
            step += 1;

            let hash = md5::compute(format!("{}{}", self.inp, step));

            if Day4::confirm_padding(hash, "00000") {
                break;
            }
        }

        let ans = step.to_string();

        println!("\nAnswer to first question is {}!\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut step = 0;

        loop {
            step += 1;

            let hash = md5::compute(format!("{}{}", self.inp, step));

            if Day4::confirm_padding(hash, "000000") {
                break;
            }
        }

        let ans = step.to_string();

        println!("\nAnswer to second question is {}!\n", ans.green());

        Ok(ans)
    }
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 {
            inp: String::new()
        }
    }

    pub fn confirm_padding(v: Digest, padding: &str) -> bool {
        format!("{:x}", v).starts_with(padding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("609043");

        let mut day4 = Day4::new();

        day4.init("inputs/2015/4a.txt")
            .expect("error trying to init day{}");

        let q1 = day4.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("6742839");

        let mut day4 = Day4::new();

        day4.init("inputs/2015/4a.txt")
            .expect("error trying to init day{day}");

        let q2 = day4.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
