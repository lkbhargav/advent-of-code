use std::collections::HashMap;

use crate::prelude::*;

pub struct Day1 {
    left: Vec<u64>,
    right: Vec<u64>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2024/1.txt";
// const TEST_FILE_NAME: &str = "inputs/2024/1a.txt";

#[derive(Clone, Debug)]
struct Holder {
    frequency: u64,
    occurences: u64,
}

impl Questions for Day1 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        contents.lines().filter(|f| !f.is_empty()).for_each(|d| {
            if d.is_empty() {
                return;
            }

            let s = d.split_once(" ").unwrap();

            self.left
                .push(s.0.trim().parse().expect("error parsing left number"));

            self.right
                .push(s.1.trim().parse().expect("error parsing right number"));
        });

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.left.sort();
        self.right.sort();

        let mut diff = 0;

        for id in 0..self.left.len() {
            diff += self.left[id].abs_diff(self.right[id]);
        }

        let ans = diff.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.left.sort();
        self.right.sort();

        let mut map: HashMap<u64, Holder> = HashMap::new();
        let mut pointer_idx = 0;

        for left in &self.left {
            if map.contains_key(&left) {
                let mut val = map.get(&left).unwrap().clone();

                val.occurences += 1;
                map.insert(*left, val);
                continue;
            }

            let mut frequency = 0;

            loop {
                if *left != self.right[pointer_idx] {
                    if *left < self.right[pointer_idx] {
                        break;
                    }

                    pointer_idx += 1;
                    continue;
                }

                pointer_idx += 1;
                frequency += 1;
            }

            map.insert(
                *left,
                Holder {
                    frequency,
                    occurences: 1,
                },
            );
        }

        let mut ans = 0;

        for (key, value) in map {
            ans += key * value.frequency * value.occurences;
        }

        let ans = ans.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            left: vec![],
            right: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("11");

        let mut day1 = Day1::new();

        day1.init("inputs/2024/1a.txt")
            .expect("error trying to init day1");

        let q1 = day1.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("31");

        let mut day1 = Day1::new();

        day1.init("inputs/2024/1a.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
