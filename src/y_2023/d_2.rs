use crate::prelude::*;

pub struct Day2 {
    games: Vec<Game>,
}

#[derive(Debug, Clone)]
pub struct Game {
    id: usize,
    rounds: Vec<(u16, u16, u16)>, // (u16, u16, u16) => (red, green, blue)
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2023/2.txt";
// const TEST_FILE_NAME: &str = "inputs/2023/2a.txt";

impl Questions for Day2 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let games = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|l| {
                let v = l.split_once(":").expect("tried to spilt once at `:`");
                let id =
                    v.0.split_once(" ")
                        .expect("trying to split once at the whitespace")
                        .1
                        .parse::<usize>()
                        .expect("trying to parse string as usize");
                let rounds = v.1.trim().split(";").collect::<Vec<&str>>();

                let mut rounds_holder = vec![];

                for round in rounds {
                    let sub_rounds = round.split(",").map(|v| v.trim()).collect::<Vec<&str>>();

                    let mut holder = (0, 0, 0);

                    for sub_round in sub_rounds {
                        let vals = sub_round.split_once(" ").expect("trying to split at space");
                        let color_val = vals
                            .0
                            .parse::<u16>()
                            .expect("trying to parse string to u16");

                        let color = vals.1.to_lowercase();

                        match color.as_str() {
                            "red" => holder.0 = color_val,
                            "green" => holder.1 = color_val,
                            "blue" => holder.2 = color_val,
                            _ => panic!("invalid color found"),
                        };
                    }

                    rounds_holder.push(holder);
                }

                Game {
                    id,
                    rounds: rounds_holder,
                }
            })
            .collect::<Vec<Game>>();

        self.games = games;

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let (pr, pg, pb) = &(12, 13, 14); // (red, green, blue)

        let mut sum_of_ids = 0;

        for game in &self.games {
            let mut passed = 0;

            for (r, g, b) in &game.rounds {
                if r <= pr && g <= pg && b <= pb {
                    passed += 1;
                }
            }

            if passed == game.rounds.len() {
                sum_of_ids += game.id;
            }
        }

        let ans = sum_of_ids.to_string();

        println!("\nAnswer to first question is {}\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut total = 0;

        for game in &self.games {
            let (mut lnr, mut lng, mut lnb) = (0, 0, 0);

            for (r, g, b) in &game.rounds {
                if *r > lnr || lnr == 0 {
                    lnr = *r;
                }

                if *g > lng || lng == 0 {
                    lng = *g;
                }

                if *b > lnb || lnb == 0 {
                    lnb = *b;
                }
            }

            let product = lnr * lng * lnb;

            total += product;
        }

        let ans = total.to_string();

        println!("\nAnswer to second question is {}\n", ans.green());

        Ok(ans)
    }
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 { games: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("8");

        let mut day2 = Day2::new();

        day2.init("inputs/2023/2a.txt")
            .expect("error trying to init day2");

        let q1 = day2.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("2286");

        let mut day2 = Day2::new();

        day2.init("inputs/2023/2a.txt")
            .expect("error trying to init day2");

        let q2 = day2.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
