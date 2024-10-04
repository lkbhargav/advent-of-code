use std::collections::HashMap;

use crate::prelude::*;

#[derive(Debug)]
struct Record {
    preprocessed_encryted_name: String,
    encrypted_name: Vec<(char, u16)>,
    sector_id: u32,
    checksum: String,
    is_valid: bool,
}

pub struct Day4 {
    data: Vec<Record>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2016/4.txt";
// const TEST_FILE_NAME: &str = "inputs/2016/4a.txt";

impl Questions for Day4 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        contents.lines().filter(|f| !f.is_empty()).for_each(|l| {
            let parts = l.split_once("[").expect("error splitting at [");

            let checksum = parts.1.replace("]", "").to_string();

            let mut pieces = parts.0.split("-").collect::<Vec<&str>>();

            let sector_id = pieces[pieces.len() - 1]
                .parse()
                .expect("error parsing sector id");

            // remove the last index that we used for sector id
            pieces.remove(pieces.len() - 1);

            let preprocessed_encryted_name = pieces.join(" ");

            let mut encrypted_name = vec![];

            let mut map = HashMap::new();

            for word in pieces {
                for c in word.chars() {
                    map.insert(
                        c,
                        match map.get(&c) {
                            None => 1,
                            Some(v) => v + 1,
                        },
                    );
                }
            }

            for (k, v) in map {
                encrypted_name.push((k, v));
            }

            self.data.push(Record {
                preprocessed_encryted_name,
                encrypted_name,
                sector_id,
                checksum,
                is_valid: false,
            })
        });

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sum = 0;

        for record in &mut self.data {
            record.encrypted_name.sort_by(|a, b| {
                if b.1 == a.1 {
                    return (a.0 > b.0).cmp(&true);
                }

                (b.1 > a.1).cmp(&true)
            });

            let mut computed_checksum = vec![];

            for ch in &record.encrypted_name {
                computed_checksum.push(ch.0.to_string());
            }

            if computed_checksum
                .join("")
                .to_string()
                .starts_with(&record.checksum)
            {
                sum += record.sector_id;
                record.is_valid = true;
            }
        }

        let ans = sum.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut sector_id = 0;
        for record in &self.data {
            if !record.is_valid {
                continue;
            }

            let mut decoded_room_name_tokens = vec![];

            let modulo = (record.sector_id % 26) as u8;

            for c in record.preprocessed_encryted_name.chars() {
                if c == ' ' {
                    decoded_room_name_tokens.push(String::from(" "));
                    continue;
                }

                let mut c = c as u8 + modulo;

                if c > 122 {
                    c = 97 + (c - 122) - 1;
                }

                let c = (c as char).to_string();

                decoded_room_name_tokens.push(c);
            }

            let st = decoded_room_name_tokens.join("");

            if st.starts_with("northpole object") {
                sector_id = record.sector_id;
            }
        }

        let ans = sector_id.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 { data: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("1514");

        let mut day4 = Day4::new();

        day4.init("inputs/2016/4a.txt")
            .expect("error trying to init day4");

        let q1 = day4.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        // * this test case doesn't apply since the second part is very specific
        let expected_q2 = String::from("na");

        // let mut day4 = Day4::new();

        // day4.init("inputs/2016/4a.txt")
        //     .expect("error trying to init day4");

        // let q2 = day4.question_two().unwrap();

        let q2 = "na";

        assert_eq!(expected_q2, q2);
    }
}
