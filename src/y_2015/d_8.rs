use crate::prelude::*;
use regex::Regex;

pub struct Day8 {
    inp: Vec<String>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "/inputs/2015/8.txt";

impl Questions for Day8 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.inp = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|v| v.to_string())
            .collect();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let pattern =
            Regex::new(r"(\\x)(?P<val>[0-9a-f]{2})").expect("error loading regex pattern");

        let mut code_data = 0;
        let mut string_data = 0;

        for v in &self.inp {
            code_data += v.len();

            let v = v.strip_prefix(r#"""#).unwrap();
            let v = v.strip_suffix(r#"""#).unwrap();

            let v = pattern.replace_all(v, format!("x"));

            let v = v.replace(r#"\\"#, r"x").replace(r#"\""#, "x");

            string_data += v.len();
        }

        let res = code_data - string_data;

        let ans = res.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut code_data = 0;
        let mut newly_encoded_data = 0;

        for v in &self.inp {
            code_data += v.len();

            newly_encoded_data += v.len() + 2 + v.matches("\\").count() + v.matches("\"").count();
        }

        let res = newly_encoded_data - code_data;

        let ans = res.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day8 {
    pub fn new() -> Day8 {
        Day8 { inp: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("15");

        let mut day8 = Day8::new();

        day8.init("inputs/2015/8a.txt")
            .expect("error trying to init day8");

        let q1 = day8.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q1b_works() {
        let expected_q1 = String::from("12");

        let mut day8 = Day8::new();

        day8.init("inputs/2015/8b.txt")
            .expect("error trying to init day8");

        let q1 = day8.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("19");

        let mut day8 = Day8::new();

        day8.init("inputs/2015/8b.txt")
            .expect("error trying to init day8");

        let q2 = day8.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }

    #[test]
    fn q2b_works() {
        let expected_q2 = String::from("25");

        let mut day8 = Day8::new();

        day8.init("inputs/2015/8a.txt")
            .expect("error trying to init day8");

        let q2 = day8.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
