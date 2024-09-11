use crate::prelude::*;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn parse(d: &str) -> Direction {
        match d {
            "D" => Direction::Down,
            "U" => Direction::Up,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!("invalid direction input"),
        }
    }
}

pub struct Day2 {
    data: Vec<Vec<Direction>>,
    grid: Vec<Vec<u8>>,
    q2_grid: Vec<Vec<char>>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2016/2.txt";
// const TEST_FILE_NAME: &str = "inputs/2016/2a.txt";

impl Questions for Day2 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.data = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|i| {
                i.trim()
                    .split("")
                    .filter(|i| !i.is_empty())
                    .map(|d| Direction::parse(d))
                    .collect::<Vec<Direction>>()
            })
            .collect::<Vec<Vec<Direction>>>();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut code: Vec<String> = vec![];

        let mut start_pos = (1, 1);

        for row in &self.data {
            for direction in row {
                match direction {
                    Direction::Down => {
                        if start_pos.0 < self.grid.len() - 1 {
                            start_pos.0 += 1
                        }
                    }
                    Direction::Up => {
                        if start_pos.0 > 0 {
                            start_pos.0 -= 1
                        }
                    }
                    Direction::Left => {
                        if start_pos.1 > 0 {
                            start_pos.1 -= 1
                        }
                    }
                    Direction::Right => {
                        if start_pos.1 < self.grid.len() - 1 {
                            start_pos.1 += 1
                        }
                    }
                };
            }

            code.push((self.grid[start_pos.0][start_pos.1]).to_string());
        }

        let ans = code.join("").to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = vec![];

        let mut start_pos = (2, 0);

        for v in &self.data {
            for direction in v {
                match direction {
                    Direction::Up => {
                        if start_pos.0 > 0 {
                            start_pos.0 -= 1;
                        }

                        if self.q2_grid[start_pos.0][start_pos.1] == ' ' {
                            start_pos.0 += 1;
                        }
                    }
                    Direction::Down => {
                        if start_pos.0 < self.q2_grid.len() - 1 {
                            start_pos.0 += 1
                        }

                        if self.q2_grid[start_pos.0][start_pos.1] == ' ' {
                            start_pos.0 -= 1;
                        }
                    }
                    Direction::Right => {
                        if start_pos.1 < self.q2_grid.len() - 1 {
                            start_pos.1 += 1
                        }

                        if self.q2_grid[start_pos.0][start_pos.1] == ' ' {
                            start_pos.1 -= 1;
                        }
                    }
                    Direction::Left => {
                        if start_pos.1 > 0 {
                            start_pos.1 -= 1
                        }

                        if self.q2_grid[start_pos.0][start_pos.1] == ' ' {
                            start_pos.1 += 1;
                        }
                    }
                }
            }

            ans.push(self.q2_grid[start_pos.0][start_pos.1]);
        }

        let ans = ans
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("");

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 {
            data: vec![],
            grid: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            q2_grid: vec![
                vec![' ', ' ', '1', ' ', ' '],
                vec![' ', '2', '3', '4', ' '],
                vec!['5', '6', '7', '8', '9'],
                vec![' ', 'A', 'B', 'C', ' '],
                vec![' ', ' ', 'D', ' ', ' '],
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("1985");

        let mut day2 = Day2::new();

        day2.init("inputs/2016/2a.txt")
            .expect("error trying to init day2");

        let q1 = day2.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("5DB3");

        let mut day2 = Day2::new();

        day2.init("inputs/2016/2a.txt")
            .expect("error trying to init day2");

        let q2 = day2.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
