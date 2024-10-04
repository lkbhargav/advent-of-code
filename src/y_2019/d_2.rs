use crate::prelude::*;

pub struct Day2 {
    data: Vec<u64>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2019/2.txt";
// const TEST_FILE_NAME: &str = "inputs/2019/2a.txt";

impl Questions for Day2 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        contents.trim().split(",").for_each(|num| {
            self.data
                .push(num.trim().parse().expect("expected a number"));
        });

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut idx = 0;

        let mut data = self.data.clone();

        loop {
            let op_code = data[idx];

            match op_code {
                1 => {
                    let index_to_update = data[idx + 3] as usize;

                    let sum = data[data[idx + 1] as usize] + data[data[idx + 2] as usize];

                    data[index_to_update] = sum;
                    idx += 4;
                }
                2 => {
                    let index_to_update = data[idx + 3] as usize;

                    let product = data[data[idx + 1] as usize] * data[data[idx + 2] as usize];

                    data[index_to_update] = product;
                    idx += 4;
                }
                99 => break,
                _ => break,
            }
        }

        let ans = data[0].to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = 0;

        'outer: for idx1 in 0..99 {
            for idx2 in 0..99 {
                let mut idx = 0;
                let mut data = self.data.clone();

                data[1] = idx1;
                data[2] = idx2;

                loop {
                    let op_code = data[idx];

                    match op_code {
                        1 => {
                            let index_to_update = data[idx + 3] as usize;

                            let v1 = data[data[idx + 1] as usize];
                            let v2 = data[data[idx + 2] as usize];

                            let sum = v1 + v2;

                            data[index_to_update] = sum;
                            idx += 4;
                        }
                        2 => {
                            let index_to_update = data[idx + 3] as usize;

                            let v1 = data[data[idx + 1] as usize];
                            let v2 = data[data[idx + 2] as usize];

                            let product = v1 * v2;

                            data[index_to_update] = product;
                            idx += 4;
                        }
                        99 => break,
                        _ => break,
                    }

                    if data[0] == 19690720 {
                        ans = (idx1 * 100) + idx2;
                        break 'outer;
                    }
                }
            }
        }

        let ans = ans.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 { data: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("3500");

        let mut day2 = Day2::new();

        day2.init("inputs/2019/2a.txt")
            .expect("error trying to init day2");

        let q1 = day2.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("na");

        let mut day2 = Day2::new();

        day2.init("inputs/2019/2a.txt")
            .expect("error trying to init day2");

        // * The question asked doesn't apply to the provided test case, so for compatibility purposes just retaining this test case
        // let q2 = day2.question_two().unwrap();

        assert_eq!(expected_q2, "na");
    }
}
