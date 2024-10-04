use std::collections::HashSet;

use crate::prelude::*;

pub struct Day1 {
    data: Vec<u32>,
    in_question: u32,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2020/1.txt";
// const TEST_FILE_NAME: &str = "inputs/2020/1a.txt";

impl Questions for Day1 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        contents.lines().filter(|f| !f.is_empty()).for_each(|num| {
            self.data.push(
                num.trim()
                    .parse()
                    .expect("expected a number but found a string"),
            );
        });

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut set = HashSet::new();
        let mut ans = 0;

        for num in &self.data {
            if *num > self.in_question {
                continue;
            }

            let diff = self.in_question - num;

            if set.contains(&diff) {
                ans = diff * num;
                break;
            }

            set.insert(num);
        }

        let ans = ans.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.data.sort();

        let mut ans = 1;

        'outer: for (idx, first_num) in self.data.iter().enumerate() {
            if *first_num > self.in_question {
                break;
            }

            let updated_in_question = self.in_question - *first_num;

            let mut left = idx + 1;
            let mut right = self.data.len() - 1;

            loop {
                let to_compare = self.data[left] + self.data[right];

                if to_compare == updated_in_question {
                    ans *= self.data[left] * first_num * self.data[right];
                    break 'outer;
                } else if to_compare < updated_in_question {
                    left += 1;
                } else {
                    right -= 1;
                }

                if left >= right {
                    break;
                }
            }
        }

        let ans = ans.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            data: vec![],
            in_question: 2020,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("514579");

        let mut day1 = Day1::new();

        day1.init("inputs/2020/1a.txt")
            .expect("error trying to init day1");

        let q1 = day1.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("241861950");

        let mut day1 = Day1::new();

        day1.init("inputs/2020/1a.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
