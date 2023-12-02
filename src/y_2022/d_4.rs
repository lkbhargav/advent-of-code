use crate::prelude::*;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair(usize, usize);

pub struct Day4 {
    inp: Vec<(Pair, Pair)>,
}

impl Questions for Day4 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.inp = contents
            .lines()
            .filter(|v| !v.is_empty())
            .map(|v| {
                let pattern = Regex::new(
                    format!("(?P<p1a>\\d+)-(?P<p1b>\\d+),(?P<p2a>\\d+)-(?P<p2b>\\d+)").as_str(),
                )
                .expect("error inputing regex expression");

                let captures = pattern
                    .captures(v)
                    .expect("error trying to get the captures");

                let p1a = captures
                    .name("p1a")
                    .expect("error trying to get the pair 1, 1st part");

                let p1b = captures
                    .name("p1b")
                    .expect("error trying to get the pair 1, 2nd part");

                let p2a = captures
                    .name("p2a")
                    .expect("error trying to get the pair 2, 1st part");

                let p2b = captures
                    .name("p2b")
                    .expect("error trying to get the pair 2, 2nd part");

                (
                    Pair(p1a.as_str().parse().unwrap(), p1b.as_str().parse().unwrap()),
                    Pair(p2a.as_str().parse().unwrap(), p2b.as_str().parse().unwrap()),
                )
            })
            .collect();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut number_of_overlaps = 0;

        self.inp.clone().into_iter().for_each(|v| {
            if (v.0 .0 >= v.1 .0 && v.0 .1 <= v.1 .1) || (v.1 .0 >= v.0 .0 && v.1 .1 <= v.0 .1) {
                number_of_overlaps += 1;
            }
        });

        let ans = number_of_overlaps.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut number_of_overlaps = 0;

        self.inp.clone().into_iter().for_each(|v| {
            if ((v.0 .0 >= v.1 .0 && v.0 .0 <= v.1 .1) || (v.0 .1 >= v.1 .0 && v.0 .1 <= v.1 .1))
                || ((v.1 .0 >= v.0 .0 && v.1 .0 <= v.0 .1)
                    || (v.1 .1 >= v.0 .0 && v.1 .1 <= v.0 .1))
            {
                number_of_overlaps += 1;
            }
        });

        let ans = number_of_overlaps.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 { inp: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("2");

        let mut day4 = Day4::new();

        day4.init("inputs/2022/4a.txt")
            .expect("error trying to init day4");

        let q1 = day4.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("4");

        let mut day4 = Day4::new();

        day4.init("inputs/2022/4a.txt")
            .expect("error trying to init day4");

        let q2 = day4.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
