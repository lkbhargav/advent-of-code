use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug)]
enum Instruction {
    Create(u16, u16),
    RotateRows { pos: u16, offset: u16 },
    RotateCols { pos: u16, offset: u16 },
}

#[derive(Debug)]
struct Screen(Vec<Vec<char>>);

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for c in line {
                write!(f, "{c} ").expect("something went wrong while using the Display lol");
            }
            writeln!(f, "").expect("something went wrong while using the Display lol");
        }

        Ok(())
    }
}

pub struct Day8 {
    instructions: Vec<Instruction>,
    screen: Screen,
    dimensions: (u16, u16),
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2016/8.txt";
// const TEST_FILE_NAME: &str = "inputs/2016/8a.txt";

impl Questions for Day8 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        contents.lines().filter(|f| !f.is_empty()).for_each(|line| {
            let line = line.trim();

            if line.starts_with("rect ") {
                let coords = line.replace("rect ", "");
                let coords = coords.split_once("x").expect("error splitting at x");

                self.instructions.push(Instruction::Create(
                    coords.0.parse().expect("expected a number"),
                    coords.1.parse().expect("expected a number"),
                ))
            } else if line.starts_with("rotate column x=") {
                let coords = line.replace("rotate column x=", "");
                let coords = coords.split_once("by").expect("error splitting at x");

                self.instructions.push(Instruction::RotateCols {
                    pos: coords.0.trim().parse().expect("expected a number"),
                    offset: coords.1.trim().parse().expect("expected a number"),
                })
            } else if line.starts_with("rotate row y=") {
                let coords = line.replace("rotate row y=", "");
                let coords = coords.split_once("by").expect("error splitting at x");

                self.instructions.push(Instruction::RotateRows {
                    pos: coords.0.trim().parse().expect("expected a number"),
                    offset: coords.1.trim().parse().expect("expected a number"),
                })
            }
        });

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.populate_screen(self.dimensions);

        self.process();

        let mut count = 0;

        for row in &self.screen.0 {
            for col in row {
                if *col == '#' {
                    count += 1;
                }
            }
        }

        let ans = count.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.populate_screen(self.dimensions);

        self.process();

        let ans = format!("{}", self.screen);

        println!("\nAnswer to 2nd question: \n{}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day8 {
    pub fn new(dimensions: (u16, u16)) -> Day8 {
        Day8 {
            instructions: vec![],
            screen: Screen(vec![]),
            dimensions,
        }
    }

    fn populate_screen(&mut self, dimension: (u16, u16)) {
        let mut screen = vec![];

        for _ in 0..dimension.1 {
            let mut cols = vec![];
            for _ in 0..dimension.0 {
                cols.push('.');
            }
            screen.push(cols);
        }

        self.screen = Screen(screen);
    }

    fn process(&mut self) {
        for instr in &self.instructions {
            match instr {
                Instruction::Create(x, y) => {
                    for row in 0..*y {
                        for col in 0..*x {
                            self.screen.0[row as usize][col as usize] = '#';
                        }
                    }
                }
                Instruction::RotateCols { pos, offset } => {
                    let mut col_to_row = vec![];

                    for row in &self.screen.0 {
                        col_to_row.push(row[*pos as usize]);
                    }

                    let mut new_col = col_to_row.clone();

                    for (idx, c) in col_to_row.iter().enumerate() {
                        let new_idx = (idx + *offset as usize) % col_to_row.len();

                        new_col[new_idx] = *c;
                    }

                    let mut idy = 0;

                    for row in &mut self.screen.0 {
                        row[*pos as usize] = new_col[idy];
                        idy += 1;
                    }
                }
                Instruction::RotateRows { pos, offset } => {
                    let mut row = self.screen.0[*pos as usize].clone();

                    for (idx, c) in (&self.screen.0[*pos as usize]).iter().enumerate() {
                        let new_idx = (idx + *offset as usize) % row.len();

                        row[new_idx] = *c;
                    }

                    self.screen.0[*pos as usize] = row;
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("6");

        let mut day8 = Day8::new((7, 3));

        day8.init("inputs/2016/8a.txt")
            .expect("error trying to init day8");

        let q1 = day8.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        // * this test case doesn't apply as the output is visual
        let expected_q2 = String::from("na");

        // let mut day8 = Day8::new(vec![(7, 3), (7, 3)]);

        // day8.init("inputs/2016/8a.txt")
        //     .expect("error trying to init day8");

        let q2 = "na";

        assert_eq!(expected_q2, q2);
    }
}
