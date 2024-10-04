use crate::prelude::*;

pub struct Day1 {
    data: Vec<u32>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2019/1.txt";
// const TEST_FILE_NAME: &str = "inputs/2019/1a.txt";

impl Questions for Day1 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        contents.lines().filter(|f| !f.is_empty()).for_each(|num| {
            let num = num
                .trim()
                .parse()
                .expect("expected a number but found a string");

            self.data.push(num);
        });

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;

        for n in &self.data {
            Day1::logic(*n as f32, &mut sum);
        }

        let ans = sum.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;

        for n in &self.data {
            let mut n = *n as f32;

            loop {
                n = Day1::logic(n, &mut sum) as f32;

                if n <= 0f32 {
                    break;
                }
            }
        }

        let ans = sum.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 { data: vec![] }
    }

    fn logic(n: f32, sum: &mut u32) -> u32 {
        let div = (n / 3f32).floor() as u32;
        if div <= 2 {
            return 0;
        }

        let res = div - 2;
        *sum += res;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("34241");

        let mut day1 = Day1::new();

        day1.init("inputs/2019/1a.txt")
            .expect("error trying to init day1");

        let q1 = day1.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("51316");

        let mut day1 = Day1::new();

        day1.init("inputs/2019/1a.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
