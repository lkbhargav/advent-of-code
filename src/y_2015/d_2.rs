use crate::prelude::*;

pub struct Dimensions {
    l: u32,
    w: u32,
    h: u32,
}

impl Dimensions {
    pub fn new(l: u32, w: u32, h: u32) -> Dimensions {
        Dimensions { l, w, h }
    }
}

pub struct Day2 {
    inp: Vec<Dimensions>,
}

impl Questions for Day2 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let values = contents
            .lines()
            .map(|v| {
                let v = v
                    .trim()
                    .split("x")
                    .filter(|v| !v.is_empty())
                    .map(|v| {
                        v.parse::<u32>()
                            .expect("error trying to convert string to int")
                    })
                    .collect::<Vec<_>>();
                Dimensions::new(v[0], v[1], v[2])
            })
            .collect::<Vec<Dimensions>>();

        self.inp = values;

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = String::new();

        let mut res = 0;

        for d in &self.inp {
            let (least_area, total) = Day2::get_dimensions(d);
            res = least_area + total + res;
        }

        ans = res.to_string();

        println!("\nAnswer to first question is {}!\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = String::new();

        let mut res = 0;

        for d in &self.inp {
            let (wrap, bow) = Day2::get_ribbon_for_wrap_and_bow(d);
            res = wrap + bow + res;
        }

        ans = res.to_string();

        println!("\nAnswer to second question is {}!\n", ans.green());

        Ok(ans)
    }
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 { inp: vec![] }
    }

    pub fn get_dimensions(d: &Dimensions) -> (usize, usize) {
        let dimensions = vec![d.l * d.h, d.h * d.w, d.w * d.l];

        let least_value = *dimensions
            .iter()
            .min()
            .expect("error trying to find the min value") as usize;

        let area = dimensions.iter().map(|v| 2 * v).sum::<u32>() as usize;

        (least_value, area)
    }

    pub fn get_ribbon_for_wrap_and_bow(d: &Dimensions) -> (usize, usize) {
        let mut dimensions = vec![d.l, d.h, d.w];
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

        let mut day2 = Day2::new();

        day2.init("inputs/2015/2a.txt")
            .expect("error trying to init day2");

        let q1 = day2.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("34");

        let mut day2 = Day2::new();

        day2.init("inputs/2015/2a.txt")
            .expect("error trying to init day2");

        let q2 = day2.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
