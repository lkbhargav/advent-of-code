use crate::prelude::*;
use std::str;

#[derive(Debug, Clone, Default)]
pub struct Password {
    password: Vec<u8>,
    valid: bool,
}

impl Password {
    pub fn new(password: &str) -> Self {
        let password = password.chars().map(|c| c as u8).collect::<Vec<u8>>();

        let mut pass = Self {
            password,
            valid: false,
        };

        pass.validate();

        pass
    }

    pub fn generate_new_password(&mut self) {
        let mut last_index = self.password.len() - 1;

        loop {
            if self.password[last_index] + 1 - 97 == 26 {
                self.password[last_index] = 97;
                last_index -= 1;
            } else {
                self.password[last_index] = self.password[last_index] + 1;
                break;
            }
        }
    }

    pub fn validate(&mut self) {
        let mut first_rule = false;
        let mut found_a_pair = false;
        let mut pair_counter = 0;
        let disallowed_characters = &[105u8, 108, 111];
        let mut disallowed_chacacters_flag = true;

        for (idx, c) in self.password.iter().enumerate() {
            if disallowed_characters.contains(c) {
                disallowed_chacacters_flag = false;
                break;
            }

            if !first_rule && idx + 2 < self.password.len() {
                if c + 1 == self.password[idx + 1] && c + 2 == self.password[idx + 2] {
                    first_rule = true;
                }
            }

            if pair_counter != 2 && idx + 1 < self.password.len() {
                if found_a_pair {
                    found_a_pair = false;
                    continue;
                }

                if *c == self.password[idx + 1] {
                    found_a_pair = true;
                    pair_counter += 1;
                }
            }
        }

        self.valid = first_rule && disallowed_chacacters_flag && pair_counter == 2;
    }

    pub fn to_string(&self) -> String {
        str::from_utf8(&self.password).unwrap().to_string()
    }
}

pub struct Day11 {
    inp: Password,
    is_test: bool,
}

// uncomment the following line incase you want to get the file name to reintialize
const FILE_NAME: &str = "inputs/2015/11.txt";

impl Questions for Day11 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.inp = Password::new(&contents);

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let ans = self.new_password();

        let ans = ans;

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut file_name = FILE_NAME;
        if self.is_test {
            file_name = "inputs/2015/11a.txt";
        }

        self.init(file_name)
            .expect("error trying to reinitialize data");

        self.new_password();

        let ans = self.new_password();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day11 {
    pub fn new(is_test: bool) -> Day11 {
        Day11 {
            inp: Default::default(),
            is_test,
        }
    }

    pub fn new_password(&mut self) -> String {
        loop {
            self.inp.generate_new_password();
            self.inp.validate();

            if self.inp.valid {
                break;
            }
        }

        self.inp.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("abcdffaa");

        let mut day11 = Day11::new(true);

        day11
            .init("inputs/2015/11a.txt")
            .expect("error trying to init day11");

        let q1 = day11.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("abcdffbb");

        let mut day11 = Day11::new(true);

        day11
            .init("inputs/2015/11a.txt")
            .expect("error trying to init day11");

        let q2 = day11.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
