use crate::prelude::*;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Day12 {
    start: (i32, i32),
    end: (i32, i32),
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
                    self.start = (idx as i32, idy as i32);
                } else if char == end_point {
                    self.end = (idx as i32, idy as i32);
                }

                tmp.push(char);
            }

            self.map.push(tmp);
        }

        dbg!(self.map.clone());

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        // let pq = PriorityQueue::new();

        let ans = self.find_path(self.start, 0, HashSet::new());

        let ans = ans.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        // TODO: your logic goes in here...

        let ans = "na".to_string();

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

    pub fn find_path(
        &self,
        next_pos: (i32, i32),
        count: usize,
        already_visited: HashSet<(i32, i32)>,
    ) -> usize {
        let nx = next_pos.0;
        let ny = next_pos.1;

        let mut already_visited = already_visited;
        let mut count = count;

        for np in &self.adjacent_locs {
            let new_x = nx + np.0;
            let new_y = ny + np.1;

            let new_loc = (new_x, new_y);

            if new_x < 0 || new_y < 0 || already_visited.contains(&new_loc) {
                continue;
            }

            let mut val = self.map[nx as usize][ny as usize];

            if val == 'S' as u8 {
                val = 'a' as u8;
            }

            if new_loc == self.end {
                return count;
            }

            let new_val = self.map[new_x as usize][new_y as usize];

            if val == new_val + 1 || val == new_val {
                already_visited.insert(new_loc);
                count = self.find_path(new_loc, count + 1, already_visited.clone());
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
        let expected_q2 = String::from("na");

        let mut day12 = Day12::new();

        day12
            .init("inputs/2022/12a.txt")
            .expect("error trying to init day12");

        let q2 = day12.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
