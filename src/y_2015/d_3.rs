use crate::prelude::*;

pub struct Day3 {}

impl Questions for Day3 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        // TODO: file parsing logic goes here

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = String::new();

        // TODO: your logic goes in here...

        println!("\nAnswer to first question is {}!\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = String::new();

        // TODO: your logic goes in here...

        println!("\nAnswer to second question is {}!\n", ans.green());

        Ok(ans)
    }
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("");

        let mut day3 = Day3::new();

        day3.init("inputs/2015/3a.txt")
            .expect("error trying to init day{}");

        let q1 = day3.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("");

        let mut day3 = Day3::new();

        day3.init("inputs/2015/3a.txt")
            .expect("error trying to init day{day}");

        let q2 = day3.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
