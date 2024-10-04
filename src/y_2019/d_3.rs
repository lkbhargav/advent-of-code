use std::{
    collections::{HashMap, HashSet},
    i32, vec,
};

use crate::prelude::*;

#[derive(Debug, Clone)]
enum Vector {
    Up(u16),
    Down(u16),
    Right(u16),
    Left(u16),
}

impl Vector {
    fn parse(inp: &str) -> Vector {
        let split_data = inp.split_at(1);

        let magnitude = split_data.1.trim().parse().expect("expected a number");

        match split_data.0.trim() {
            "U" => Vector::Up(magnitude),
            "D" => Vector::Down(magnitude),
            "R" => Vector::Right(magnitude),
            "L" => Vector::Left(magnitude),
            _ => panic!("invalid input"),
        }
    }
}

pub struct Day3 {
    data: (Vec<Vector>, Vec<Vector>),
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2019/3.txt";
// const TEST_FILE_NAME: &str = "inputs/2019/3a.txt";

impl Questions for Day3 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let data = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|line| {
                line.split(",")
                    .map(|v| Vector::parse(v))
                    .collect::<Vec<Vector>>()
            })
            .collect::<Vec<Vec<Vector>>>();

        self.data.0 = data.get(0).expect("expected it to present").to_vec();
        self.data.1 = data.get(1).expect("expected it to present").to_vec();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut map = HashSet::new();

        let mut pos = (0, 0);

        for vector in &self.data.0 {
            match vector {
                Vector::Up(n) => {
                    for _ in 0..*n {
                        pos.0 += 1;
                        map.insert(pos);
                    }
                }
                Vector::Down(n) => {
                    for _ in 0..*n {
                        pos.0 -= 1;
                        map.insert(pos);
                    }
                }
                Vector::Right(n) => {
                    for _ in 0..*n {
                        pos.1 += 1;
                        map.insert(pos);
                    }
                }
                Vector::Left(n) => {
                    for _ in 0..*n {
                        pos.1 -= 1;
                        map.insert(pos);
                    }
                }
            }
        }

        let mut overlaps = vec![];

        pos = (0, 0);

        for vector in &self.data.1 {
            match vector {
                Vector::Up(n) => {
                    for _ in 0..*n {
                        pos.0 += 1;
                        if map.contains(&pos) {
                            overlaps.push(pos);
                        }
                    }
                }
                Vector::Down(n) => {
                    for _ in 0..*n {
                        pos.0 -= 1;
                        if map.contains(&pos) {
                            overlaps.push(pos);
                        }
                    }
                }
                Vector::Right(n) => {
                    for _ in 0..*n {
                        pos.1 += 1;
                        if map.contains(&pos) {
                            overlaps.push(pos);
                        }
                    }
                }
                Vector::Left(n) => {
                    for _ in 0..*n {
                        pos.1 -= 1;
                        if map.contains(&pos) {
                            overlaps.push(pos);
                        }
                    }
                }
            }
        }

        let mut shorest_distance_from_overlap_to_source = i32::MAX;

        for overlap in overlaps {
            let distance = (overlap.0 - 0i32).abs() + (overlap.1 - 0i32).abs();
            if distance < shorest_distance_from_overlap_to_source {
                shorest_distance_from_overlap_to_source = distance;
            }
        }

        let ans = shorest_distance_from_overlap_to_source.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut map = HashMap::new();

        let mut pos = (0, 0);

        let mut counter = 0;

        for vector in &self.data.0 {
            match vector {
                Vector::Up(n) => {
                    for _ in 0..*n {
                        pos.0 += 1;
                        counter += 1;
                        map.insert(pos, counter);
                    }
                }
                Vector::Down(n) => {
                    for _ in 0..*n {
                        pos.0 -= 1;
                        counter += 1;
                        map.insert(pos, counter);
                    }
                }
                Vector::Right(n) => {
                    for _ in 0..*n {
                        pos.1 += 1;
                        counter += 1;
                        map.insert(pos, counter);
                    }
                }
                Vector::Left(n) => {
                    for _ in 0..*n {
                        pos.1 -= 1;
                        counter += 1;
                        map.insert(pos, counter);
                    }
                }
            }
        }

        let mut overlaps = HashMap::new();

        pos = (0, 0);

        let mut counter = 0;

        for vector in &self.data.1 {
            match vector {
                Vector::Up(n) => {
                    for _ in 0..*n {
                        pos.0 += 1;
                        counter += 1;
                        match map.get(&pos) {
                            None => (),
                            Some(count) => {
                                overlaps.insert(pos, (count, counter));
                            }
                        };
                    }
                }
                Vector::Down(n) => {
                    for _ in 0..*n {
                        pos.0 -= 1;
                        counter += 1;
                        match map.get(&pos) {
                            None => (),
                            Some(count) => {
                                overlaps.insert(pos, (count, counter));
                            }
                        };
                    }
                }
                Vector::Right(n) => {
                    for _ in 0..*n {
                        pos.1 += 1;
                        counter += 1;
                        match map.get(&pos) {
                            None => (),
                            Some(count) => {
                                overlaps.insert(pos, (count, counter));
                            }
                        };
                    }
                }
                Vector::Left(n) => {
                    for _ in 0..*n {
                        pos.1 -= 1;
                        counter += 1;
                        match map.get(&pos) {
                            None => (),
                            Some(count) => {
                                overlaps.insert(pos, (count, counter));
                            }
                        };
                    }
                }
            }
        }

        let mut minimum_steps = i32::MAX;

        for (_position, step_counts) in overlaps {
            let num_of_steps = step_counts.0 + step_counts.1;

            if num_of_steps < minimum_steps {
                minimum_steps = num_of_steps;
            }
        }

        let ans = minimum_steps.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {
            data: (vec![], vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("6");

        let mut day3 = Day3::new();

        day3.init("inputs/2019/3a.txt")
            .expect("error trying to init day3");

        let q1 = day3.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q1_works_2() {
        let expected_q1 = String::from("159");

        let mut day3 = Day3::new();

        day3.init("inputs/2019/3b.txt")
            .expect("error trying to init day3");

        let q1 = day3.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q1_works_3() {
        let expected_q1 = String::from("135");

        let mut day3 = Day3::new();

        day3.init("inputs/2019/3c.txt")
            .expect("error trying to init day3");

        let q1 = day3.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("30");

        let mut day3 = Day3::new();

        day3.init("inputs/2019/3a.txt")
            .expect("error trying to init day3");

        let q2 = day3.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }

    #[test]
    fn q2_works_2() {
        let expected_q2 = String::from("610");

        let mut day3 = Day3::new();

        day3.init("inputs/2019/3b.txt")
            .expect("error trying to init day3");

        let q2 = day3.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }

    #[test]
    fn q2_works_3() {
        let expected_q2 = String::from("410");

        let mut day3 = Day3::new();

        day3.init("inputs/2019/3c.txt")
            .expect("error trying to init day3");

        let q2 = day3.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
