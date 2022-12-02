use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Pieces {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Pieces {
    fn convert(i: &str) -> Pieces {
        match i {
            "a" | "A" | "x" | "X" => Pieces::Rock,
            "b" | "B" | "y" | "Y" => Pieces::Paper,
            "c" | "C" | "z" | "Z" => Pieces::Scissors,
            _ => panic!("invalid type passed, {}", i),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameResult {
    Win = 6,
    Lose = 0,
    Draw = 3,
}

pub struct Day2 {
    inp: Vec<(Pieces, Pieces)>,
}

impl Questions for Day2 {
    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = String::new();

        let mut score = 0;

        for fight in &self.inp {
            let res = Day2::game_result(fight.clone());
            score += res as u32 + fight.1 as u32;
        }

        ans = score.to_string();

        println!("\nAnswer to first question is {}!\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = String::new();

        let mut score = 0;

        for fight in &self.inp {
            let res = Day2::me_to_result(fight.clone().1);
            score += res as u32 + Day2::what_should_i_play(fight.clone().0, res) as u32;
        }

        ans = score.to_string();

        println!("\nAnswer to second question is {}!\n", ans.green());

        Ok(ans)
    }

    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let contents = contents
            .lines()
            .map(|v| {
                let v = v.split_whitespace().collect::<Vec<&str>>();
                (Pieces::convert(v[0]), Pieces::convert(v[1]))
            })
            .collect::<Vec<(Pieces, Pieces)>>();

        self.inp = contents;

        Ok(())
    }
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 { inp: vec![] }
    }

    pub fn game_result(fight: (Pieces, Pieces)) -> GameResult {
        let or = fight.0;
        let mr = fight.1;

        if or == mr {
            return GameResult::Draw;
        }

        if (or == Pieces::Rock && mr == Pieces::Scissors)
            || (or == Pieces::Paper && mr == Pieces::Rock)
            || (or == Pieces::Scissors && mr == Pieces::Paper)
        {
            return GameResult::Lose;
        }

        GameResult::Win
    }

    pub fn me_to_result(me: Pieces) -> GameResult {
        match me {
            Pieces::Rock => GameResult::Lose,
            Pieces::Paper => GameResult::Draw,
            Pieces::Scissors => GameResult::Win,
        }
    }

    pub fn what_should_i_play(opponent: Pieces, game_result: GameResult) -> Pieces {
        match game_result {
            GameResult::Lose => {
                return match opponent {
                    Pieces::Rock => Pieces::Scissors,
                    Pieces::Paper => Pieces::Rock,
                    Pieces::Scissors => Pieces::Scissors,
                };
            }
            GameResult::Win => {
                return match opponent {
                    Pieces::Rock => Pieces::Paper,
                    Pieces::Paper => Pieces::Scissors,
                    Pieces::Scissors => Pieces::Rock,
                };
            }
            GameResult::Draw => {
                return opponent;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("15");

        let mut day2 = Day2::new();

        day2.init("inputs/2022/2a.txt")
            .expect("error trying to init day2");

        let q1 = day2.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("12");

        let mut day2 = Day2::new();

        day2.init("inputs/2022/2a.txt")
            .expect("error trying to init day2");

        let q2 = day2.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
