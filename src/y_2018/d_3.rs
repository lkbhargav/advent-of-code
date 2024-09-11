use std::collections::HashMap;

use crate::prelude::*;

pub struct Day3 {
    data: Vec<(usize, (usize, usize), (usize, usize))>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2018/3.txt";
// const TEST_FILE_NAME: &str = "inputs/2018/3a.txt";

impl Questions for Day3 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.data = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|l| {
                let parts = l.split_whitespace().collect::<Vec<&str>>();
                let binding = parts[2].replace(":", "");
                let start_coords = binding
                    .split_once(",")
                    .expect("something went wrong parsing the start coords");

                let start_coords = (
                    start_coords.0.parse::<usize>().unwrap(),
                    start_coords.1.parse::<usize>().unwrap(),
                );

                let matrix = parts[3]
                    .split_once("x")
                    .expect("something went wrong trying to parse matrix");

                let matrix = (
                    matrix.0.parse::<usize>().unwrap(),
                    matrix.1.parse::<usize>().unwrap(),
                );

                let id = parts[0]
                    .replace("#", "")
                    .parse::<usize>()
                    .expect("error parsing the id");

                (id, start_coords, matrix)
            })
            .collect::<Vec<(usize, (usize, usize), (usize, usize))>>();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();

        for v in &self.data {
            let start_coords = v.1;
            let matrix = v.2;

            for h in 0..matrix.0 {
                for v in 0..matrix.1 {
                    let key = (start_coords.0 + h, start_coords.1 + v);
                    let val = match map.get(&key) {
                        None => 1,
                        Some(d) => d + 1,
                    };

                    map.insert(key, val);
                }
            }
        }

        let mut count = 0;

        for (_, value) in map {
            if value >= 2 {
                count += 1;
            }
        }

        let ans = count.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut map: HashMap<(usize, usize), usize> = HashMap::new();

        for v in &self.data {
            let start_coords = v.1;
            let matrix = v.2;

            for h in 0..matrix.0 {
                for v in 0..matrix.1 {
                    let key = (start_coords.0 + h, start_coords.1 + v);
                    let val = match map.get(&key) {
                        None => 1,
                        Some(d) => d + 1,
                    };

                    map.insert(key, val);
                }
            }
        }

        let mut ans = 0;

        'outer: for v in &self.data {
            let id = v.0;
            let start_coords = v.1;
            let matrix = v.2;

            for h in 0..matrix.0 {
                for v in 0..matrix.1 {
                    let key = (start_coords.0 + h, start_coords.1 + v);
                    let val = match map.get(&key) {
                        None => 0,
                        Some(d) => *d,
                    };

                    if val > 1 {
                        continue 'outer;
                    }
                }
            }

            ans = id;
            break;
        }

        let ans = ans.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 { data: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("4");

        let mut day3 = Day3::new();

        day3.init("inputs/2018/3a.txt")
            .expect("error trying to init day3");

        let q1 = day3.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("3");

        let mut day3 = Day3::new();

        day3.init("inputs/2018/3a.txt")
            .expect("error trying to init day3");

        let q2 = day3.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
