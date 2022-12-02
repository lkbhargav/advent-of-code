use crate::{prelude::*, util::read_from_file};

pub struct Day01 {
    inp: Vec<u32>,
}

impl Questions for Day01 {
    fn question_one(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Question 1!");

        println!("{}", &self.inp[0]);

        Ok(())
    }

    fn question_two(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Question 2!");

        println!("{}", &self.inp[0]+&self.inp[1]+&self.inp[2]);

        Ok(())
    }
}

impl Day01 {
    pub fn new() -> Day01 {
        let input = read_from_file("inputs/2022/01.txt");

        let mut elves_input = input
            .split("\n\n")
            .into_iter()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|v| {
                v.split("\n")
                    .filter(|v| !v.is_empty())
                    .map(|v| {
                        v.parse::<u32>()
                            .expect("something went wrong converting string to int")
                    })
                    .reduce(|a, b| a + b)
                    .unwrap()
            })
            .collect::<Vec<u32>>();

        elves_input.sort_by(|a, b| b.partial_cmp(a).unwrap());

        return Day01 { inp: elves_input };
    }
}
