use std::collections::HashSet;

use crate::prelude::*;

const DOT: u8 = 46;
const NUM_START_RANGE: u8 = 48;
const NUM_END_RANGE: u8 = 57;
const PRODUCT: u8 = 42;

pub struct Day3 {
    data: Vec<Vec<u8>>,
    all_side_constants: Vec<(isize, isize)>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2023/3.txt";
// const TEST_FILE_NAME: &str = "inputs/2023/3a.txt";

impl Questions for Day3 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let data = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|l| l.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        self.data = data;

        // these constant values when added to `x` and `y` coords will give us the diagonals of that position
        let mut diagonal_constants = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];

        // these constant values when added to `x` and `y` coords will give us the adjacents of that position
        let mut adjacent_constants = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        adjacent_constants.append(&mut diagonal_constants);

        self.all_side_constants = adjacent_constants;

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;

        for (idx, vec) in self.data.iter().enumerate() {
            let mut num = vec![];
            let mut is_part_number = false;

            for (jdx, d) in vec.iter().enumerate() {
                if *d >= NUM_START_RANGE && *d <= NUM_END_RANGE {
                    num.push(*d);

                    let idx = idx as isize;
                    let jdx = jdx as isize;

                    if !is_part_number {
                        for c in &self.all_side_constants {
                            if idx + c.0 >= 0
                                && jdx + c.1 >= 0
                                && idx + c.0 < self.data.len() as isize
                                && jdx + c.1 < vec.len() as isize
                            {
                                let pos_val = self.data[(idx + c.0) as usize][(jdx + c.1) as usize];

                                if !(pos_val == DOT
                                    || (pos_val >= NUM_START_RANGE && pos_val <= NUM_END_RANGE))
                                {
                                    is_part_number = true;
                                }
                            }
                        }
                    }
                } else {
                    if is_part_number {
                        let n = std::str::from_utf8(num.as_slice())
                            .expect("converting num slice to number")
                            .parse::<usize>()
                            .expect("converting str to usize");

                        sum += n;
                    }
                    num.clear();
                    is_part_number = false;
                }

                if jdx + 1 == vec.len() {
                    if is_part_number {
                        let n = std::str::from_utf8(num.as_slice())
                            .expect("converting num slice to number")
                            .parse::<usize>()
                            .expect("converting str to usize");

                        sum += n;
                    }
                }
            }
        }

        let ans = sum.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;

        for (idx, vec) in self.data.iter().enumerate() {
            for (jdx, d) in vec.iter().enumerate() {
                if *d == PRODUCT {
                    let idx = idx as isize;
                    let jdx = jdx as isize;

                    let mut numbers_to_mul = HashSet::new();

                    for c in &self.all_side_constants {
                        if idx + c.0 >= 0
                            && idx + c.0 < self.data.len() as isize
                            && jdx + c.1 >= 0
                            && jdx + c.1 < vec.len() as isize
                        {
                            let (nx, ny) = ((idx + c.0) as usize, (jdx + c.1) as usize);

                            let nd = self.data[nx][ny];

                            if nd >= NUM_START_RANGE && nd <= NUM_END_RANGE {
                                numbers_to_mul.insert(self.fetch_number((nx, ny)));
                            }
                        }
                    }

                    if numbers_to_mul.len() != 2 {
                        continue;
                    }

                    let mut prod = 1;

                    for k in numbers_to_mul {
                        prod *= k;
                    }

                    sum += prod;
                }
            }
        }

        let ans = sum.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {
            data: vec![],
            all_side_constants: vec![],
        }
    }

    fn fetch_number(&self, coords: (usize, usize)) -> usize {
        let mut num = vec![];

        let row = &self.data[coords.0];

        let mut num_start_pos = coords.1;

        while num_start_pos > 0 {
            let tmp = num_start_pos - 1;

            if row[tmp] >= NUM_START_RANGE && row[tmp] <= NUM_END_RANGE {
                num_start_pos = tmp;
                continue;
            }

            break;
        }

        num.push((row[num_start_pos] as char).to_string());

        while num_start_pos + 1 < row.len() {
            let tmp = num_start_pos + 1;

            if row[tmp] >= NUM_START_RANGE && row[tmp] <= NUM_END_RANGE {
                num_start_pos = tmp;
                num.push((row[num_start_pos] as char).to_string());
                continue;
            }

            break;
        }

        num.join("").parse::<usize>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("4361");

        let mut day3 = Day3::new();

        day3.init("inputs/2023/3a.txt")
            .expect("error trying to init day3");

        let q1 = day3.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("467835");

        let mut day3 = Day3::new();

        day3.init("inputs/2023/3a.txt")
            .expect("error trying to init day3");

        let q2 = day3.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
