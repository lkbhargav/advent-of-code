use std::collections::HashMap;

use crate::prelude::*;

pub struct Day3 {
    inp: Vec<u8>,
    q2: Vec<u8>,
}

impl Questions for Day3 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);
        let mut common_values = vec![];

        contents.lines().filter(|f| !f.is_empty()).for_each(|v| {
            let mut map = HashMap::new();

            let mid_point = v.len() / 2;

            for (i, c) in v.chars().enumerate() {
                if i > mid_point - 1 {
                    let does_contain = map.contains_key(&c);

                    if does_contain {
                        let val = Day3::fetch_value(c);

                        common_values.push(val);

                        break;
                    }
                } else {
                    map.insert(c, 1);
                }
            }
        });

        let mut line_num = 0;
        let mut map = HashMap::new();
        let mut common_values_q2 = vec![];

        contents.lines().filter(|f| !f.is_empty()).for_each(|v| {
            if line_num % 3 == 0 {
                line_num = 0;
                map = HashMap::new();
            }

            let mut tmp = vec![];

            for c in v.chars() {
                if !tmp.contains(&c) {
                    let v = match map.get(&c) {
                        Some(v) => *v,
                        None => 0,
                    };

                    // this is line 3 since we start the line numbers from 0
                    if line_num == 2 {
                        if v == 2 {
                            let val = Day3::fetch_value(c);
                            common_values_q2.push(val);
                            break;
                        }
                    }

                    map.insert(c, v + 1);

                    tmp.push(c);
                }
            }

            line_num += 1;
        });

        self.inp = common_values;
        self.q2 = common_values_q2;

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;
        self.inp.clone().into_iter().for_each(|v| {
            sum += v as usize;
        });

        let ans = sum.to_string();

        println!("\nAnswer to first question is {}!\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;
        
        self.q2.clone().into_iter().for_each(|v| {
            sum += v as usize;
        });

        let ans = sum.to_string();

        println!("\nAnswer to second question is {}!\n", ans.green());

        Ok(ans)
    }
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {
            inp: vec![],
            q2: vec![],
        }
    }

    pub fn fetch_value(c: char) -> u8 {
        let mut val = c as u8;
        if c.is_lowercase() {
            val = val - 96;
        } else {
            val = (val - 65) + 27;
        }

        val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("157");

        let mut day3 = Day3::new();

        day3.init("inputs/2022/3a.txt")
            .expect("error trying to init day3");

        let q1 = day3.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("70");

        let mut day3 = Day3::new();

        day3.init("inputs/2022/3a.txt")
            .expect("error trying to init day3");

        let q2 = day3.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
