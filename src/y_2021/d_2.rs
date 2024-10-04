use crate::prelude::*;

#[derive(Debug)]
enum SubmarineMovement {
    Forward(isize),
    Down(isize),
    Up(isize),
}

pub struct Day2 {
    data: Vec<SubmarineMovement>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2021/2.txt";
// const TEST_FILE_NAME: &str = "inputs/2021/2a.txt";

impl Questions for Day2 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|l| l.trim().split_once(" ").unwrap())
            .for_each(|(dir, val)| {
                let val = val.parse().expect("error parsing value");
                self.data.push(match dir {
                    "forward" => SubmarineMovement::Forward(val),
                    "down" => SubmarineMovement::Down(val),
                    _ => SubmarineMovement::Up(val),
                });
            });

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut direction = 0;
        let mut altitude = 0;

        for submarine_movement in &self.data {
            match submarine_movement {
                SubmarineMovement::Forward(v) => direction += v,
                SubmarineMovement::Down(v) => altitude += v,
                SubmarineMovement::Up(v) => altitude -= v,
            };
        }

        let ans = direction * altitude;

        let ans = ans.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut direction = 0;
        let mut altitude = 0;
        let mut aim = 0;

        for submarine_movement in &self.data {
            match submarine_movement {
                SubmarineMovement::Forward(v) => {
                    direction += v;
                    altitude += v * aim;
                }
                SubmarineMovement::Down(v) => aim += v,
                SubmarineMovement::Up(v) => aim -= v,
            };
        }

        let ans = (direction * altitude).to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 { data: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("150");

        let mut day2 = Day2::new();

        day2.init("inputs/2021/2a.txt")
            .expect("error trying to init day2");

        let q1 = day2.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("900");

        let mut day2 = Day2::new();

        day2.init("inputs/2021/2a.txt")
            .expect("error trying to init day2");

        let q2 = day2.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
