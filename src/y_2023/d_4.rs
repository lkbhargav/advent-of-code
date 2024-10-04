use std::collections::{HashMap, HashSet};

use crate::prelude::*;

#[derive(Debug)]
struct ScratchCardInfo {
    card_number: u16,
    winning_numbers: HashSet<u8>,
    my_numbers: Vec<u8>,
}

pub struct Day4 {
    data: Vec<ScratchCardInfo>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2023/4.txt";
// const TEST_FILE_NAME: &str = "inputs/2023/4a.txt";

impl Questions for Day4 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        contents.lines().filter(|f| !f.is_empty()).for_each(|l| {
            let scratch_card = l.split_once(":").expect("error trying to split at :");

            let card_number = scratch_card
                .0
                .split_once(" ")
                .expect("error splitting at space while fetching the card number")
                .1
                .trim()
                .parse::<u16>()
                .expect("error trying to parse the card number to u16");

            let numbers_part = scratch_card
                .1
                .trim()
                .split_once("|")
                .expect("error trying to split at |");

            let mut winning_numbers = HashSet::new();

            numbers_part
                .1
                .trim()
                .split(" ")
                .filter(|c| !c.is_empty())
                .for_each(|c| {
                    winning_numbers.insert(
                        c.trim()
                            .parse::<u8>()
                            .expect("error trying to parse the number"),
                    );
                });

            let my_numbers = numbers_part
                .0
                .trim()
                .split(" ")
                .filter(|c| !c.is_empty())
                .map(|c| c.trim().parse().expect("error parsing number"))
                .collect();

            self.data.push(ScratchCardInfo {
                card_number,
                winning_numbers,
                my_numbers,
            });
        });

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut total = 0;

        for scratchcard_info in &self.data {
            let mut count = 0;

            for num in &scratchcard_info.my_numbers {
                if scratchcard_info.winning_numbers.contains(&num) {
                    count += 1;
                }
            }

            let points = match count {
                0 => 0,
                c => 2i32.pow(c - 1),
            };

            total += points;
        }

        let ans = total.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let total_cards = self.data.len();

        let mut map = HashMap::new();

        for scratchcard_info in &self.data {
            map.insert(scratchcard_info.card_number, 1);
        }

        for scratchcard_info in &self.data {
            let mut matching_numbers_count = 0;

            for num in &scratchcard_info.my_numbers {
                if scratchcard_info.winning_numbers.contains(&num) {
                    matching_numbers_count += 1;
                }
            }

            for n in (scratchcard_info.card_number + 1)
                ..=(scratchcard_info.card_number + matching_numbers_count)
            {
                if n <= total_cards as u16 {
                    map.insert(
                        n,
                        *map.get(&n).unwrap() + *map.get(&scratchcard_info.card_number).unwrap(),
                    );
                }
            }
        }

        let mut total_scratch_cards = 0;

        for (_, v) in map {
            total_scratch_cards += v;
        }

        let ans = total_scratch_cards.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 { data: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("13");

        let mut day4 = Day4::new();

        day4.init("inputs/2023/4a.txt")
            .expect("error trying to init day4");

        let q1 = day4.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("30");

        let mut day4 = Day4::new();

        day4.init("inputs/2023/4a.txt")
            .expect("error trying to init day4");

        let q2 = day4.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
