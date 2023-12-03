use std::collections::HashMap;

use crate::prelude::*;

pub struct Day2 {
    data: Vec<Vec<char>>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2018/2.txt";
// const TEST_FILE_NAME: &str = "inputs/2018/2a.txt";

impl Questions for Day2 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let data = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|l| {
                let l = l.trim();
                l.chars().collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        self.data = data;

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut two_count = 0;
        let mut three_count = 0;

        for d in &self.data {
            let mut map = HashMap::new();
            let mut found_2 = false;
            let mut found_3 = false;

            for c in d {
                match map.get(c) {
                    None => map.insert(c, 1),
                    Some(v) => map.insert(c, v + 1),
                };
            }

            for val in map.values() {
                if *val == 2 {
                    found_2 = true;
                }

                if *val == 3 {
                    found_3 = true;
                }
            }

            if found_2 {
                two_count += 1;
            }

            if found_3 {
                three_count += 1;
            }
        }

        let ans = (two_count * three_count).to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut found = Vec::new();

        'outer: for i in 0..self.data.len() {
            for j in i + 1..self.data.len() {
                let set_1 = &self.data[i];
                let set_2 = &self.data[j];

                let zipped = set_1
                    .iter()
                    .zip(set_2.iter())
                    .collect::<Vec<(&char, &char)>>();

                let mut single_mismatch = false;

                found.clear();

                for (c1, c2) in &zipped {
                    if *c1 != *c2 && !single_mismatch {
                        single_mismatch = true;
                        continue;
                    }

                    if *c1 != *c2 && single_mismatch {
                        break;
                    }

                    found.push(*c1);
                }

                if found.len() == zipped.len() - 1 {
                    break 'outer;
                }
            }
        }

        let ans = found.into_iter().collect::<String>();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 { data: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("12");

        let mut day2 = Day2::new();

        day2.init("inputs/2018/2a.txt")
            .expect("error trying to init day2");

        let q1 = day2.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("fgij");

        let mut day2 = Day2::new();

        day2.init("inputs/2018/2b.txt")
            .expect("error trying to init day2");

        let q2 = day2.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
