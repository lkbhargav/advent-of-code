use std::{collections::HashSet, panic};

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Direction {
    Right(i32),
    Left(i32),
}

impl Direction {
    pub fn new(d: &str) -> Direction {
        if d.starts_with("L") {
            let num = d.split("L").into_iter().collect::<Vec<&str>>()[1].to_i32("L part");
            return Direction::Left(num);
        } else if d.starts_with("R") {
            let num = d.split("R").into_iter().collect::<Vec<&str>>()[1].to_i32("R part");
            return Direction::Right(num);
        }

        panic!("invalid data passed: {d}");
    }
}

pub struct Day1 {
    directions: Vec<Direction>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2016/1.txt";
// const TEST_FILE_NAME: &str = "inputs/2016/1a.txt";

impl Questions for Day1 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.directions = contents
            .split(",")
            .map(|a| {
                let a = a.trim();

                Direction::new(a)
            })
            .collect::<Vec<Direction>>();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut x = 0;
        let mut y = 0;
        // -1 => left | 1 => right | -2 => south | 2 => north
        let mut d = 2; // initial direction will be north

        for direction in &self.directions {
            match direction {
                Direction::Left(v) => match d {
                    -1 => {
                        d = -2;
                        y -= v;
                    }
                    1 => {
                        d = 2;
                        y += v;
                    }
                    -2 => {
                        d = 1;
                        x += v;
                    }
                    2 => {
                        d = -1;
                        x -= v;
                    }
                    _ => panic!("invalid op"),
                },
                Direction::Right(v) => match d {
                    -1 => {
                        d = 2;
                        y += v;
                    }
                    1 => {
                        d = -2;
                        y -= v;
                    }
                    -2 => {
                        d = -1;
                        x -= v;
                    }
                    2 => {
                        d = 1;
                        x += v;
                    }
                    _ => panic!("invalid op"),
                },
            }
        }

        if x < 0 {
            x = 0 - x;
        }

        if y < 0 {
            y = 0 - y;
        }

        let ans = x + y;

        let ans = ans.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut x = 0;
        let mut y = 0;
        // -1 => left | 1 => right | -2 => south | 2 => north
        let mut d = 2; // initial direction will be north

        let mut prev_x;
        let mut prev_y;

        let mut set = HashSet::new();

        for direction in &self.directions {
            prev_x = x;
            prev_y = y;

            match direction {
                Direction::Left(v) => match d {
                    -1 => {
                        d = -2;
                        y -= v;
                    }
                    1 => {
                        d = 2;
                        y += v;
                    }
                    -2 => {
                        d = 1;
                        x += v;
                    }
                    2 => {
                        d = -1;
                        x -= v;
                    }
                    _ => panic!("invalid op"),
                },
                Direction::Right(v) => match d {
                    -1 => {
                        d = 2;
                        y += v;
                    }
                    1 => {
                        d = -2;
                        y -= v;
                    }
                    -2 => {
                        d = -1;
                        x -= v;
                    }
                    2 => {
                        d = 1;
                        x += v;
                    }
                    _ => panic!("invalid op"),
                },
            }

            for nx in prev_x..=x {
                for ny in prev_y..=y {
                    println!("{:?}", (nx, ny));
                    if set.contains(&(nx, ny)) {
                        x = nx;
                        y = ny;
                        break;
                    }

                    set.insert((nx, ny));
                }
            }
        }

        dbg!(x, y);

        if x < 0 {
            x = 0 - x;
        }

        if y < 0 {
            y = 0 - y;
        }

        let ans = x + y;

        let ans = ans.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 { directions: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("12");

        let mut day1 = Day1::new();

        day1.init("inputs/2016/1a.txt")
            .expect("error trying to init day1");

        let q1 = day1.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("4");

        let mut day1 = Day1::new();

        day1.init("inputs/2016/1b.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
