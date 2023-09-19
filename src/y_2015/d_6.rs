use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Operation {
    pub fn parse(operation: &str) -> std::result::Result<Self, String> {
        match operation {
            "turn on" => Ok(Operation::TurnOn),
            "turn off" => Ok(Operation::TurnOff),
            "toggle" => Ok(Operation::Toggle),
            _ => Err(String::from("Invalid Operation passed")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord(usize, usize);

impl Coord {
    pub fn parse(coord: &str) -> std::result::Result<Coord, String> {
        let coord = coord
            .split(",")
            .map(|v| v.parse::<usize>().expect("error converting string to int"))
            .collect::<Vec<usize>>();

        if coord.len() != 2 {
            return Err(format!(
                "expected the coord to be the size of 2 but found {}",
                coord.len()
            ));
        }

        Ok(Coord(coord[0], coord[1]))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    operation: Operation,
    start: Coord,
    end: Coord,
}

pub struct Day6 {
    inp: Vec<Instruction>,
    lights: Vec<Vec<usize>>,
}

impl Questions for Day6 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        const OPERATION_NAME: &str = "operation";
        const START_NAME: &str = "start";
        const END_NAME: &str = "end";

        let regex_parser = RegexParser::new(format!("(?P<{OPERATION_NAME}>turn off|turn on|toggle) (?P<{START_NAME}>\\d+,\\d+) through (?P<{END_NAME}>\\d+,\\d+)").as_str());

        self.inp = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|v| {
                let captures = regex_parser.parse(v);

                let operation = captures.get_name(OPERATION_NAME);
                let operation =
                    Operation::parse(&operation).expect("error parsing string to operation");

                let start = captures.get_name(START_NAME);
                let start = Coord::parse(&start).expect("error parsing start coord");

                let end = captures.get_name(END_NAME);
                let end = Coord::parse(&end).expect("error parsing end coord");

                Instruction {
                    operation,
                    start,
                    end,
                }
            })
            .collect();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.inp.clone().into_iter().for_each(|instruction| {
            self.follow_instruction_q1(instruction);
        });

        let ans = self.number_of_lights_turned_on().to_string();

        println!("\nAnswer to first question is {}\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        // reinitialize or we will get an error since we are using the
        self.lights = vec![vec![0; 1000]; 1000];

        self.inp.clone().into_iter().for_each(|instruction| {
            self.follow_instruction_q2(instruction);
        });

        let ans = self.number_of_lights_turned_on().to_string();

        println!("\nAnswer to second question is {}\n", ans.green());

        Ok(ans)
    }
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 {
            inp: vec![],
            lights: vec![vec![0; 1000]; 1000],
        }
    }

    pub fn follow_instruction_q1(&mut self, instruction: Instruction) {
        for row in 0..self.lights.len() {
            for column in 0..self.lights[row].len() {
                if (row >= instruction.start.0 && row <= instruction.end.0)
                    && (column >= instruction.start.1 && column <= instruction.end.1)
                {
                    match instruction.operation {
                        Operation::TurnOn => self.lights[row][column] = 1,
                        Operation::TurnOff => self.lights[row][column] = 0,
                        Operation::Toggle => {
                            if self.lights[row][column] == 0 {
                                self.lights[row][column] = 1;
                            } else {
                                self.lights[row][column] = 0;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn follow_instruction_q2(&mut self, instruction: Instruction) {
        for row in 0..self.lights.len() {
            for column in 0..self.lights[row].len() {
                if (row >= instruction.start.0 && row <= instruction.end.0)
                    && (column >= instruction.start.1 && column <= instruction.end.1)
                {
                    match instruction.operation {
                        Operation::TurnOn => self.lights[row][column] += 1,
                        Operation::TurnOff => {
                            if self.lights[row][column] > 0 {
                                self.lights[row][column] -= 1;
                            }
                        }
                        Operation::Toggle => self.lights[row][column] += 2,
                    }
                }
            }
        }
    }

    pub fn number_of_lights_turned_on(&self) -> usize {
        let mut count = 0;

        for row in 0..self.lights.len() {
            for column in 0..self.lights[row].len() {
                count += self.lights[row][column];
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("998996");

        let mut day6 = Day6::new();

        day6.init("inputs/2015/6a.txt")
            .expect("error trying to init day6");

        let q1 = day6.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("1001996");

        let mut day6 = Day6::new();

        day6.init("inputs/2015/6a.txt")
            .expect("error trying to init day6");

        let q2 = day6.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }

    #[test]
    fn q2b_works() {
        let expected_q2 = String::from("2000000");

        let mut day6 = Day6::new();

        day6.init("inputs/2015/6b.txt")
            .expect("error trying to init day6");

        let q2 = day6.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
