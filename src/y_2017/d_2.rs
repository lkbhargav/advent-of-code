use crate::prelude::*;

pub struct Day2 {
    inp: Vec<Vec<u16>>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2017/2.txt";
// const TEST_FILE_NAME: &str = "inputs/2017/2a.txt";

impl Questions for Day2 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.inp = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|v| {
                let v = v.trim().split_whitespace();

                v.into_iter()
                    .map(|v| v.trim().parse().expect("error parsing string to number"))
                    .collect()
            })
            .collect();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;

        for r in self.inp.clone() {
            let row_vec = r.iter();
            sum += row_vec.clone().max().expect("error finding max")
                - row_vec.min().expect("error finding min");
        }

        let ans = sum.to_string();

        println!("\nAnswer to first question is {}\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;

        for v in &mut self.inp {
            v.sort();

            'outer: for e in v.clone() {
                let e = e.clone();
                // going in reverse
                for (idx, _) in v.iter().enumerate() {
                    let u = v[v.len() - 1 - idx];
                    if u != e && u % e == 0 {
                        sum += u / e;
                        break 'outer;
                    }
                }
            }
        }

        let ans = sum.to_string();

        println!("\nAnswer to second question is {}\n", ans.green());

        Ok(ans)
    }
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 { inp: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("18");

        let mut day2 = Day2::new();

        day2.init("inputs/2017/2a.txt")
            .expect("error trying to init day2");

        let q1 = day2.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("9");

        let mut day2 = Day2::new();

        day2.init("inputs/2017/2b.txt")
            .expect("error trying to init day2");

        let q2 = day2.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
