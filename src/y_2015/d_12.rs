use crate::prelude::*;
use serde_json::{self, Value};

pub struct Day12 {
    inp: String,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2015/12.txt";

impl Questions for Day12 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.inp = contents.into_owned();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut double_quote = false;

        let mut total_value = 0;

        let mut num_str = String::new();

        for c in self.inp.chars() {
            match c as u8 {
                34 => {
                    double_quote = !double_quote;
                }
                48..=57 | 45 => {
                    if !double_quote {
                        num_str.push(c);
                    }
                }
                _ => {
                    if num_str.len() > 0 {
                        total_value += num_str.parse::<i64>().unwrap();
                        num_str.clear();
                    }
                }
            }
        }

        let ans = total_value.to_string();

        println!("\nAnswer to first question is {}\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let json: Value = serde_json::from_str(self.inp.as_str())
            .expect("error trying to parse the input to JSON");

        let val = Day12::parse(json);

        let ans = val.to_string();

        println!("\nAnswer to second question is {}\n", ans.green());

        Ok(ans)
    }
}

impl Day12 {
    pub fn new() -> Day12 {
        Day12 { inp: String::new() }
    }

    pub fn parse(v: Value) -> i64 {
        let mut sum = 0;
        if v.is_object() {
            let obj = v.as_object().expect("expected an object but got none");

            for (_, val) in (*obj).clone() {
                if val.is_object() || val.is_array() {
                    sum += Day12::parse(val.clone());
                }

                if val.is_string()
                    && val
                        .as_str()
                        .expect("expected a str but found something else")
                        == "red"
                {
                    return 0;
                }

                if val.is_i64() {
                    sum += match val.as_i64() {
                        None => 0,
                        Some(i) => i,
                    };
                }
            }
        }

        if v.is_array() {
            let arr = v
                .as_array()
                .expect("expected an array but found something else");

            for j in (*arr).clone() {
                if j.is_i64() {
                    sum += match j.as_i64() {
                        None => 0,
                        Some(i) => i,
                    };
                }

                if j.is_array() || j.is_object() {
                    sum += Day12::parse(j.clone());
                }
            }
        }

        if v.is_i64() {
            sum += match v.as_i64() {
                None => 0,
                Some(i) => i,
            };
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("6");

        let mut day12 = Day12::new();

        day12
            .init("inputs/2015/12a.txt")
            .expect("error trying to init day12");

        let q1 = day12.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("4");

        let mut day12 = Day12::new();

        day12
            .init("inputs/2015/12a.txt")
            .expect("error trying to init day12");

        let q2 = day12.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
