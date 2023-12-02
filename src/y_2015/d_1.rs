use crate::prelude::*;

pub struct Day1 {
    inp: Vec<i32>,
}

impl Questions for Day1 {
    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let ans = (&self.inp.iter().sum::<i32>()).to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = String::new();

        let mut val = 0;

        for (index, v) in self.inp.iter().enumerate() {
            val += v;

            if val == -1 {
                ans = index.to_string();
                break;
            }
        }

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let contents = contents
            .split("")
            .map(|v| {
                if v == "(" {
                    1
                } else if v == ")" {
                    -1
                } else {
                    0
                }
            })
            .collect();

        self.inp = contents;

        Ok(())
    }
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 { inp: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("3");

        let mut day1 = Day1::new();

        day1.init("inputs/2015/1a.txt")
            .expect("error trying to init day1");

        let q1 = day1.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("1");

        let mut day1 = Day1::new();

        day1.init("inputs/2015/1a.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
