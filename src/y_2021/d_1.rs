use crate::prelude::*;

pub struct Day1 {
    data: Vec<i32>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2021/1.txt";
// const TEST_FILE_NAME: &str = "inputs/2021/1a.txt";

impl Questions for Day1 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.data = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|num| num.trim().parse().expect("error parsing a number"))
            .collect();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut increased_count = 0;
        let mut prev_num = -1;

        for num in &self.data {
            if prev_num == -1 {
                prev_num = *num;
                continue;
            }

            if *num > prev_num {
                increased_count += 1;
            }

            prev_num = *num;
        }

        let ans = increased_count.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut prev = self.data[0] + self.data[1] + self.data[2];

        let mut increased_count = 0;

        for num in 3..self.data.len() {
            let now = prev + self.data[num] - self.data[num - 3];

            if now > prev {
                increased_count += 1;
            }

            prev = now;
        }

        let ans = increased_count.to_string();

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
        let expected_q1 = String::from("7");

        let mut day1 = Day1::new();

        day1.init("inputs/2021/1a.txt")
            .expect("error trying to init day1");

        let q1 = day1.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("5");

        let mut day1 = Day1::new();

        day1.init("inputs/2021/1a.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
