use crate::prelude::*;

pub struct Day5 {
    word: String,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2016/5.txt";
// const TEST_FILE_NAME: &str = "inputs/2016/5a.txt";

impl Questions for Day5 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.word = contents.trim().to_string();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut count = 0;
        let mut password = Vec::with_capacity(8);

        loop {
            let digest = md5::compute(&format!("{}{}", self.word, count));

            let hash = format!("{:x}", digest);

            if hash.starts_with("00000") {
                let hash = hash.as_bytes();

                password.push((hash[5] as char).to_string());
            }

            if password.len() == 8 {
                break;
            }

            count += 1;
        }

        let password = password.join("");

        let ans = password.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut count = 0;
        let mut inserted_count = 0;
        let mut password = Vec::with_capacity(8);

        // initialize the password vec with defaults
        for _ in 0..8 {
            password.push(String::from("_"));
        }

        loop {
            let digest = md5::compute(&format!("{}{}", self.word, count));

            let hash = format!("{:x}", digest);

            if hash.starts_with("00000") {
                let hash = hash.as_bytes();

                let pos = match (hash[5] as char).to_string().parse::<usize>() {
                    Ok(n) => n,
                    Err(_) => {
                        count += 1;
                        continue;
                    }
                };

                if pos >= 8 {
                    count += 1;
                    continue;
                }

                if password[pos] == "_" {
                    password[pos] = (hash[6] as char).to_string();
                    inserted_count += 1;
                }
            }

            if inserted_count == 8 {
                break;
            }

            count += 1;
        }

        let password = password.join("");

        let ans = password.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day5 {
    pub fn new() -> Day5 {
        Day5 {
            word: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("18f47a30");

        let mut day5 = Day5::new();

        day5.init("inputs/2016/5a.txt")
            .expect("error trying to init day5");

        let q1 = day5.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("05ace8e3");

        let mut day5 = Day5::new();

        day5.init("inputs/2016/5a.txt")
            .expect("error trying to init day5");

        let q2 = day5.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
