
use crate::prelude::*;

pub struct Day4 {}

impl Questions for Day4 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        // TODO: file parsing logic goes here

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans= String::new();

        // TODO: your logic goes in here...

        println!("\nAnswer to first question is {}!\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans= String::new();

        // TODO: your logic goes in here...

        println!("\nAnswer to second question is {}!\n", ans.green());

        Ok(ans)
    }
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("");

        let mut day4 = Day4::new();

        day4.init("inputs/2015/4a.txt")
            .expect("error trying to init day{}");

        let q1 = day4.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("");
        
        let mut day4 = Day4::new();

        day4.init("inputs/2015/4a.txt")
            .expect("error trying to init day{day}");

        let q2 = day4.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
        
