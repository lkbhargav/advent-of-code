use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opponent {
    A,
    B,
    C,
}

impl Opponent {
    fn convert(i: &str) -> Opponent {
        match i {
            "a" | "A" => Opponent::A,
            "b" | "B" => Opponent::B,
            "c" | "C" => Opponent::C,
            _ => panic!("invalid type passed, {}", i),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Me {
    X = 1,
    Y = 2,
    Z = 3,
}

impl Me {
    fn convert(i: &str) -> Me {
        match i {
            "x" | "X" => Me::X,
            "y" | "Y" => Me::Y,
            "z" | "Z" => Me::Z,
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
    inp: Vec<(Opponent, Me)>,
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
            score += res as u32 + Day2::what_should_I_play(fight.clone().0, res) as u32;
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
                (Opponent::convert(v[0]), Me::convert(v[1]))
            })
            .collect::<Vec<(Opponent, Me)>>();

        self.inp = contents;

        Ok(())
    }
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 {
            inp: vec![]
        }
    }

    pub fn game_result(fight: (Opponent, Me)) -> GameResult {
        let or = fight.0;
        let mr = fight.1;

        if (or == Opponent::A && mr == Me::X)
            || (or == Opponent::B && mr == Me::Y)
            || (or == Opponent::C && mr == Me::Z)
        {
            return GameResult::Draw;
        }

        if (or == Opponent::A && mr == Me::Z)
            || (or == Opponent::B && mr == Me::X)
            || (or == Opponent::C && mr == Me::Y)
        {
            return GameResult::Lose;
        }

        GameResult::Win
    }

    pub fn me_to_result(me: Me) -> GameResult {
        match me {
            Me::X => GameResult::Lose,
            Me::Y => GameResult::Draw,
            Me::Z => GameResult::Win,
        }
    }

    pub fn what_should_I_play(opponent: Opponent, game_result: GameResult) -> Me {
        match game_result {
            GameResult::Lose => {
                return match opponent {
                    Opponent::A => Me::Z,
                    Opponent::B => Me::X,
                    Opponent::C => Me::Y,
                };
            },
            GameResult::Win => {
                return match opponent {
                    Opponent::A => Me::Y,
                    Opponent::B => Me::Z,
                    Opponent::C => Me::X,
                };
            }
            GameResult::Draw => {
                return match opponent {
                    Opponent::A => Me::X,
                    Opponent::B => Me::Y,
                    Opponent::C => Me::Z,
                }
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

        day2.init("inputs/2022/2a.txt").expect("error trying to init day2");

        let q1 = day2.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("12");

        let mut day2 = Day2::new();

        day2.init("inputs/2022/2a.txt").expect("error trying to init day2");

        let q2 = day2.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
