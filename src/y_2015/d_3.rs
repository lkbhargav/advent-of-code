use std::collections::HashMap;

use crate::prelude::*;

#[derive(Eq, Hash, Debug, PartialEq, Clone, Copy)]
struct Direction(i32, i32);

#[derive(Eq, Hash, Debug, PartialEq, Clone, Copy)]
pub enum Compass {
    North,
    East,
    South,
    West,
}

impl Compass {
    pub fn parse(v: &str) -> Compass {
        match v {
            "^" => Self::North,
            ">" => Self::East,
            "v" => Self::South,
            "<" => Self::West,
            _ => panic!("invalid character found"),
        }
    }
}

#[derive(Eq, Debug, PartialEq, Clone)]
pub struct Record {
    x: i32,
    y: i32,
    map: HashMap<Direction, usize>,
}

impl Record {
    pub fn new() -> Self {
        Record {
            x: 0,
            y: 0,
            map: HashMap::new(),
        }
    }

    pub fn record(&mut self) {
        let direction = Direction(self.x, self.y);
        let val = match self.map.get(&direction) {
            None => 0 as usize,
            Some(k) => *k,
        };
        self.map.insert(direction, val + 1);
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn update_directions(&mut self, v: Compass) {
        match v {
            Compass::North => self.y = self.y + 1,
            Compass::East => self.x = self.x + 1,
            Compass::South => self.y = self.y - 1,
            Compass::West => self.x = self.x - 1,
        };
    }

    pub fn merge(&mut self, r: Record) {
        self.map.extend(r.map);
    }
}

pub struct Day3 {
    inp: Vec<Compass>,
}

impl Questions for Day3 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.inp = contents
            .trim()
            .split("")
            .into_iter()
            .filter(|v| !v.is_empty())
            .map(|v| Compass::parse(v))
            .collect();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut santa = Record::new();

        santa.record();

        self.inp.clone().into_iter().for_each(|v| {
            santa.update_directions(v);
            santa.record();
        });

        let ans = santa.len().to_string();

        println!("\nAnswer to first question is {}\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut santa = Record::new();
        let mut robo_santa = Record::new();

        santa.record();
        robo_santa.record();

        self.inp.clone().into_iter().enumerate().for_each(|(i, v)| {
            if i % 2 == 0 {
                robo_santa.update_directions(v);
                robo_santa.record();
            } else {
                santa.update_directions(v);
                santa.record();
            }
        });

        santa.merge(robo_santa);

        let ans = (santa.len()).to_string();

        println!("\nAnswer to second question is {}\n", ans.green());

        Ok(ans)
    }
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 { inp: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("4");

        let mut day3 = Day3::new();

        day3.init("inputs/2015/3a.txt")
            .expect("error trying to init day3");

        let q1 = day3.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q1b_works() {
        let expected_q1 = String::from("2");

        let mut day3 = Day3::new();

        day3.init("inputs/2015/3b.txt")
            .expect("error trying to init day3");

        let q1 = day3.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q1c_works() {
        let expected_q1 = String::from("2");

        let mut day3 = Day3::new();

        day3.init("inputs/2015/3c.txt")
            .expect("error trying to init day3");

        let q1 = day3.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("3");

        let mut day3 = Day3::new();

        day3.init("inputs/2015/3a.txt")
            .expect("error trying to init day3");

        let q2 = day3.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }

    #[test]
    fn q2b_works() {
        let expected_q2 = String::from("11");

        let mut day3 = Day3::new();

        day3.init("inputs/2015/3b.txt")
            .expect("error trying to init day3");

        let q2 = day3.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
