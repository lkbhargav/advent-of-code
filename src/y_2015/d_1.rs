use crate::prelude::*;

pub struct Day1 {}

impl Questions for Day1 {
    fn question_one(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Question 1!");
        Ok(())
    }

    fn question_two(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Question 2!");
        Ok(())
    }
}

impl Day1 {
    pub fn new() -> Day1 {
        return Day1 {};
    }
}
