use crate::prelude::*;
use std::{collections::HashSet, usize};

#[derive(Debug)]
pub struct Day12 {
    start: (u32, u32),
    end: (u32, u32),
    map: Vec<Vec<u8>>,
    adjacent_locs: Vec<(i32, i32)>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2022/12.txt";
// const TEST_FILE_NAME: &str = "inputs/2022/12a.txt";

impl Questions for Day12 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let start_point = 'S' as u8;
        let end_point = 'E' as u8;

        let data = contents
            .lines()
            .filter(|f| !f.is_empty())
            .collect::<Vec<&str>>();

        for (idx, d) in data.iter().enumerate() {
            let mut tmp = vec![];

            for (idy, c) in d.chars().enumerate() {
                let char = c as u8;

                if char == start_point {
                    self.start = (idx as u32, idy as u32);
                    tmp.push('a' as u8);
                } else if char == end_point {
                    self.end = (idx as u32, idy as u32);
                    tmp.push('z' as u8);
                } else {
                    tmp.push(char);
                }
            }

            self.map.push(tmp);
        }

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        // let pq = PriorityQueue::new();

        let ans = self.find_shortest_path(self.start);

        let ans = ans.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut shortest_path = u32::MAX;

        for (idx, row) in self.map.iter().enumerate() {
            for (idy, val) in row.iter().enumerate() {
                if *val == 97 {
                    let res = self.find_shortest_path((idx as u32, idy as u32));
                    if res < shortest_path && res > 0 {
                        shortest_path = res;
                    }
                }
            }
        }

        let ans = shortest_path.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day12 {
    pub fn new() -> Day12 {
        Day12 {
            end: (0, 0),
            start: (0, 0),
            map: vec![],
            adjacent_locs: vec![(0, 1), (1, 0), (-1, 0), (0, -1)],
        }
    }

    fn find_shortest_path(&self, start_loc: (u32, u32)) -> u32 {
        let mut queue = Vec::new();
        let mut visited = HashSet::new();

        let row_len = self.map.len() as i32;
        let col_len = self.map.first().expect("don't see any columns").len() as i32;

        queue.push((start_loc, 0));

        while queue.len() > 0 {
            let q_val = queue.remove(0);

            let q_val_pos = q_val.0;

            if visited.contains(&q_val_pos) {
                continue;
            }

            visited.insert(q_val_pos);

            for al in &self.adjacent_locs {
                if q_val_pos.0 as i32 + al.0 >= 0
                    && q_val_pos.0 as i32 + al.0 < row_len
                    && q_val_pos.1 as i32 + al.1 >= 0
                    && q_val_pos.1 as i32 + al.1 < col_len
                {
                    let updated_val = self.map[(q_val_pos.0 as i32 + al.0) as usize]
                        [(q_val_pos.1 as i32 + al.1) as usize];
                    let val = self.map[q_val_pos.0 as usize][q_val_pos.1 as usize];

                    let new_pos = (
                        (q_val_pos.0 as i32 + al.0) as u32,
                        (q_val_pos.1 as i32 + al.1) as u32,
                    );

                    if updated_val >= val && updated_val - val <= 1 || updated_val < val {
                        if self.end == new_pos {
                            return q_val.1 + 1;
                        }

                        // println!("{:?} -> {:?} ({})", q_val_pos, new_pos, q_val.1 + 1);

                        queue.push((new_pos, q_val.1 + 1));
                    }
                }
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("31");

        let mut day12 = Day12::new();

        day12
            .init("inputs/2022/12a.txt")
            .expect("error trying to init day12");

        let q1 = day12.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("29");

        let mut day12 = Day12::new();

        day12
            .init("inputs/2022/12a.txt")
            .expect("error trying to init day12");

        let q2 = day12.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
