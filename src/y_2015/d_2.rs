use std::thread::panicking;

use crate::prelude::*;

pub struct Day2 {
    l: u32,
    w: u32,
    h: u32,
}

impl Questions for Day2 {
    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = String::new();

        let (least_area, total) = &self.get_dimensions();

        ans = (least_area + total).to_string();

        println!("\nAnswer to first question is {}!\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = String::new();

        let (wrap, bow) = &self.get_ribbon_for_wrap_and_bow();

        ans = (wrap + bow).to_string();

        println!("\nAnswer to second question is {}!\n", ans.green());

        Ok(ans)
    }
}

impl Day2 {
    pub fn new(file: &str) -> Day2 {
        let contents = read_from_file(file);

        let values = contents
            .trim()
            .split("x")
            .filter(|v| !v.is_empty())
            .map(|v| {
                v.parse::<u32>()
                    .expect("error trying to convert string to int")
            })
            .collect::<Vec<_>>();

        if values.len() != 3 {
            panic!(
                "expected the length of values to be 3 but found {}",
                values.len()
            );
        }

        Day2 {
            l: values[0],
            w: values[1],
            h: values[2],
        }
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        let dimensions = vec![&self.l * &self.h, &self.h * &self.w, &self.w * &self.l];

        let least_value = *dimensions
            .iter()
            .min()
            .expect("error trying to find the min value") as usize;

        let area = dimensions.iter().map(|v| 2 * v).sum::<u32>() as usize;

        (least_value, area)
    }

    pub fn get_ribbon_for_wrap_and_bow(&self) -> (usize, usize) {
        let mut dimensions = vec![&self.l, &self.h, &self.w];
        let mut wrap = 0;
        let mut bow = 1;

        dimensions.iter().for_each(|a| {
            bow = bow * *a;
        });

        dimensions.sort();

        dimensions.iter().take(2).for_each(|v| {
            wrap = wrap + (*v * 2);
        });

        (wrap as usize, bow as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("58");

        let mut day2 = Day2::new("inputs/2015/2a.txt");

        let q1 = day2.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("34");

        let mut day2 = Day2::new("inputs/2015/2a.txt");

        let q2 = day2.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
