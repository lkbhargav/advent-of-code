use crate::prelude::*;

pub struct Day3 {
    data: Vec<(u16, u16, u16)>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2016/3.txt";
// const TEST_FILE_NAME: &str = "inputs/2016/3a.txt";

impl Questions for Day3 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.data = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|l| l.trim())
            .map(|l| {
                let v = l
                    .split(" ")
                    .filter(|v| !v.is_empty())
                    .map(|n| n.parse::<u16>().expect("error parsing number"))
                    .collect::<Vec<u16>>();
                (v[0], v[1], v[2])
            })
            .collect::<Vec<(u16, u16, u16)>>();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let ans = Day3::logic(&self.data).to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut new_data = vec![];

        let mut three_rows = Vec::with_capacity(3);
        // convert columns of data into rows
        for row in &self.data {
            three_rows.push(row);

            if three_rows.len() == 3 {
                new_data.push((three_rows[0].0, three_rows[1].0, three_rows[2].0));
                new_data.push((three_rows[0].1, three_rows[1].1, three_rows[2].1));
                new_data.push((three_rows[0].2, three_rows[1].2, three_rows[2].2));
                three_rows.clear();
            }
        }

        let ans = Day3::logic(&new_data).to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 { data: vec![] }
    }

    pub fn logic(data: &Vec<(u16, u16, u16)>) -> u32 {
        let mut count = 0;

        for v in data {
            if v.0 + v.1 > v.2 && v.1 + v.2 > v.0 && v.2 + v.0 > v.1 {
                count += 1;
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
        let expected_q1 = String::from("1");

        let mut day3 = Day3::new();

        day3.init("inputs/2016/3a.txt")
            .expect("error trying to init day3");

        let q1 = day3.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("2");

        let mut day3 = Day3::new();

        day3.init("inputs/2016/3a.txt")
            .expect("error trying to init day3");

        let q2 = day3.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
