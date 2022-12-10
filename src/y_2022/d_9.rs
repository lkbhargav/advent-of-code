use std::collections::HashSet;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn new(i: &str) -> Direction {
        match i {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("unexpected character found"),
        }
    }
}

pub struct Day9 {
    inp: Vec<(u16, Direction)>,
    diagonal_constants: Vec<(i32, i32)>,
    adjacent_constants: Vec<(i32, i32)>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2022/9.txt";
// const TEST_FILE_NAME: &str = "inputs/2022/9a.txt";

impl Questions for Day9 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let directions = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|f| {
                let values = f.split_whitespace().into_iter().collect::<Vec<&str>>();
                let blocks = values[1].parse().expect("expected a number");
                (blocks, Direction::new(values[0]))
            })
            .collect::<Vec<(u16, Direction)>>();

        self.inp = directions;

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut set = HashSet::new();

        let mut x = 0;
        let mut y = 0;

        let mut head;
        let mut tail = (x, y);

        set.insert(tail);

        for v in self.inp.clone() {
            for _ in 0..v.0 {
                match v.1 {
                    Direction::Down => y -= 1,
                    Direction::Up => y += 1,
                    Direction::Left => x -= 1,
                    Direction::Right => x += 1,
                }

                head = (x, y);
                tail = self.update_tail_position(head, tail);
                set.insert(tail);
            }
        }

        let ans = set.len().to_string();

        println!("\nAnswer to first question is {}\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut set = HashSet::new();

        let mut x = 0;
        let mut y = 0;

        let mut head;
        let mut rest_of_the_body = [(x, y); 9];

        set.insert(rest_of_the_body[0]);

        for v in self.inp.clone() {
            for _ in 0..v.0 {
                match v.1 {
                    Direction::Down => y -= 1,
                    Direction::Up => y += 1,
                    Direction::Left => x -= 1,
                    Direction::Right => x += 1,
                }

                head = (x, y);

                let mut tmp = head;

                for (i, tail) in rest_of_the_body.clone().iter().enumerate() {
                    tmp = self.update_tail_position(tmp, *tail);
                    rest_of_the_body[i] = tmp;
                }

                set.insert(tmp);
            }
        }

        let ans = set.len().to_string();

        println!("\nAnswer to second question is {}\n", ans.green());

        Ok(ans)
    }
}

impl Day9 {
    pub fn new() -> Day9 {
        // these constant values when added to `x` and `y` coords will give us the diagonals of that position
        let diagonal_constants = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];

        // these constant values when added to `x` and `y` coords will give us the adjacents of that position
        let adjacent_constants = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        Day9 {
            inp: vec![],
            diagonal_constants,
            adjacent_constants,
        }
    }

    pub fn update_tail_position(&self, head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
        let (hx, hy) = head_pos;
        let (mut tx, mut ty) = tail_pos;

        if self.is_touching(head_pos, tail_pos) {
            return (tx, ty);
        }

        // two spaces apart in y coords
        if hx == tx {
            if hy - ty == 2 {
                ty += 1;
                return (tx, ty);
            } else if hy - ty == -2 {
                ty -= 1;
                return (tx, ty);
            }
        }

        // two spaces apart in x coords
        if hy == ty {
            if hx - tx == 2 {
                tx += 1;
                return (tx, ty);
            } else if hx - tx == -2 {
                tx -= 1;
                return (tx, ty);
            }
        }

        for i in &self.diagonal_constants {
            let new_tail = (tx + i.0, ty + i.1);
            if self.is_touching(head_pos, new_tail) {
                return new_tail;
            }
        }

        return (tx, ty);
    }

    pub fn is_touching(&self, head_pos: (i32, i32), tail_pos: (i32, i32)) -> bool {
        let (hx, hy) = head_pos;
        let (tx, ty) = tail_pos;

        // same position
        if hx == tx && hy == ty {
            return true;
        }

        for pos in &self.adjacent_constants {
            if (hx == tx + pos.0 && hy == ty) || (hx == tx && hy == ty + pos.1) {
                return true;
            }
        }

        for pos in &self.diagonal_constants {
            if hx == tx + pos.0 && hy == ty + pos.1 {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("13");

        let mut day9 = Day9::new();

        day9.init("inputs/2022/9a.txt")
            .expect("error trying to init day9");

        let q1 = day9.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("1");

        let mut day9 = Day9::new();

        day9.init("inputs/2022/9a.txt")
            .expect("error trying to init day9");

        let q2 = day9.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
