use colored::ColoredString;

use crate::prelude::*;
use std::{fmt::Display, ops::Range};

pub struct ToDisplay(Vec<Vec<ColoredString>>);

impl ToDisplay {
    pub fn new() -> Self {
        Self(vec![vec![]])
    }
}

impl Display for ToDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for c in line {
                write!(f, "{c} ").expect("something went wrong while using the Display lol");
            }
            println!();
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ClockCircuit {
    Noop,
    Addx(i32),
}

impl ClockCircuit {
    pub fn new(op: &str) -> Self {
        let val = op.trim().split_whitespace().collect::<Vec<&str>>();

        if val.len() == 2 {
            let num = val[1]
                .parse::<i32>()
                .expect("error converting string to integer");

            return ClockCircuit::Addx(num);
        }

        ClockCircuit::Noop
    }
}

pub struct Day10 {
    instructions: Vec<ClockCircuit>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2022/10.txt";
// const TEST_FILE_NAME: &str = "inputs/2022/10a.txt";

impl Questions for Day10 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.instructions = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|v| ClockCircuit::new(v))
            .collect::<Vec<ClockCircuit>>();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut register = 1;
        let mut cycles = 0;
        let mut ans = 0;

        for instruction in &self.instructions {
            match instruction {
                ClockCircuit::Noop => {
                    ans += Day10::increment_cycles_and_calculate_val(&mut cycles, register);
                }
                ClockCircuit::Addx(i) => {
                    ans += Day10::increment_cycles_and_calculate_val(&mut cycles, register);
                    ans += Day10::increment_cycles_and_calculate_val(&mut cycles, register);
                    register += i;
                }
            };
        }

        let ans = ans.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut display = ToDisplay::new();

        let mut sprite_pos = 1;
        let mut cycles = 0;

        let mut line = vec![];

        for instruction in &self.instructions {
            let range = (sprite_pos - 1)..(sprite_pos + 2);

            match instruction {
                ClockCircuit::Noop => {
                    Day10::paint(&mut cycles, &mut line, &mut display, range);
                }
                ClockCircuit::Addx(i) => {
                    Day10::paint(&mut cycles, &mut line, &mut display, range.clone());
                    Day10::paint(&mut cycles, &mut line, &mut display, range);
                    sprite_pos += i;
                }
            };
        }

        println!("\nAnswer to 2nd question: \n");
        println!("{display}");

        let ans = "na".to_string();

        Ok(ans)
    }
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 {
            instructions: vec![],
        }
    }

    pub fn increment_cycles_and_calculate_val(cycle: &mut i32, register: i32) -> i32 {
        *cycle += 1;

        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                let ans = *cycle * register;
                ans
            }
            _ => 0,
        }
    }

    pub fn paint(
        cycle: &mut i32,
        line: &mut Vec<ColoredString>,
        display: &mut ToDisplay,
        range: Range<i32>,
    ) {
        if range.contains(&cycle) {
            line.push("#".green().bold());
        } else {
            line.push(".".black().italic());
        }

        *cycle += 1;

        if *cycle % 40 == 0 {
            *cycle = 0;
            display.0.push(line.clone());
            line.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("13140");

        let mut day10 = Day10::new();

        day10
            .init("inputs/2022/10a.txt")
            .expect("error trying to init day10");

        let q1 = day10.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    // Test for question 2 is not required as the result is supposed to be confirmed visually
}
