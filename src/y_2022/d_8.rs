use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Forest {
    height: usize,
    width: usize,
    forest_grid: Vec<Vec<u8>>,
}

impl Forest {
    pub fn new(forest_grid: Vec<Vec<u8>>) -> Self {
        let height = forest_grid.len();
        let width = forest_grid[0].len();

        Self {
            height,
            width,
            forest_grid,
        }
    }

    pub fn is_the_tree_visible(&mut self, row: usize, column: usize) -> (bool, usize) {
        let mut visible = false;

        if row == 0 || column == 0 || row == self.width - 1 || column == self.height - 1 {
            visible = true;
        }

        if visible {
            return (visible, 0);
        }

        let tree_height = self.forest_grid[row][column];

        visible = false;

        let mut count_1 = 0;
        let mut visible_1 = false;
        let mut count_2 = 0;
        let mut visible_2 = false;
        let mut count_3 = 0;
        let mut visible_3 = false;
        let mut count_4 = 0;
        let mut visible_4 = false;

        for i in (0..row).rev() {
            count_1 += 1;
            if tree_height <= self.forest_grid[i][column] {
                visible_1 = true;
                break;
            }
        }

        for i in row + 1..self.width {
            count_2 += 1;
            if tree_height <= self.forest_grid[i][column] {
                visible_2 = true;
                break;
            }
        }

        for i in (0..column).rev() {
            count_3 += 1;
            if tree_height <= self.forest_grid[row][i] {
                visible_3 = true;
                break;
            }
        }

        for i in column + 1..self.height {
            count_4 += 1;
            if tree_height <= self.forest_grid[row][i] {
                visible_4 = true;
                break;
            }
        }

        if !(visible_1 && visible_2 && visible_3 && visible_4) {
            visible = true;
        }

        (visible, count_1 * count_2 * count_3 * count_4)
    }
}

pub struct Day8 {
    inp: Forest,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2022/8.txt";
// const TEST_FILE_NAME: &str = "inputs/2022/8a.txt";

impl Questions for Day8 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let forest_grid = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|v| {
                v.chars()
                    .map(|i| {
                        i.to_string()
                            .parse::<u8>()
                            .expect("error converting string to number")
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let forest = Forest::new(forest_grid);

        self.inp = forest;

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut number_of_trees_visible = 0;

        for (ridx, _) in self.inp.clone().forest_grid.iter().enumerate() {
            for (cidx, _) in self.inp.clone().forest_grid[ridx].iter().enumerate() {
                let res = self.inp.is_the_tree_visible(ridx, cidx);

                if res.0 {
                    number_of_trees_visible += 1;
                }
            }
        }

        let ans = number_of_trees_visible.to_string();

        println!("\nAnswer to first question is {}\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut max_scenic_score_possible = usize::MIN;

        for (ridx, _) in self.inp.clone().forest_grid.iter().enumerate() {
            for (cidx, _) in self.inp.clone().forest_grid[ridx].iter().enumerate() {
                let res = self.inp.is_the_tree_visible(ridx, cidx);

                if res.0 {
                    if res.1 > max_scenic_score_possible {
                        max_scenic_score_possible = res.1;
                    }
                }
            }
        }

        let ans = max_scenic_score_possible.to_string();

        println!("\nAnswer to second question is {}\n", ans.green());

        Ok(ans)
    }
}

impl Day8 {
    pub fn new() -> Day8 {
        Day8 {
            inp: Forest::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("21");

        let mut day8 = Day8::new();

        day8.init("inputs/2022/8a.txt")
            .expect("error trying to init day8");

        let q1 = day8.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("8");

        let mut day8 = Day8::new();

        day8.init("inputs/2022/8a.txt")
            .expect("error trying to init day8");

        let q2 = day8.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
