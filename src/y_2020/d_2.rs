use crate::prelude::*;

#[derive(Debug, Default)]
struct PasswordData {
    min: u16,
    max: u16,
    character: char,
    password: String,
}

pub struct Day2 {
    data: Vec<PasswordData>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2020/2.txt";
// const TEST_FILE_NAME: &str = "inputs/2020/2a.txt";

impl Questions for Day2 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        contents.lines().filter(|f| !f.is_empty()).for_each(|l| {
            let mut pd = PasswordData::default();
            let parts = l.trim().split(" ").collect::<Vec<&str>>();
            let range = parts[0].trim().split_once("-").unwrap();
            let character = parts[1]
                .trim()
                .replace(":", "")
                .chars()
                .collect::<Vec<char>>()[0];
            let password = parts[2].trim().to_string();

            pd.min = range.0.parse().unwrap();
            pd.max = range.1.parse().unwrap();
            pd.character = character;
            pd.password = password;

            self.data.push(pd);
        });

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut valid_passwords = 0;

        for pd in &self.data {
            let mut char_occurences = 0;

            for c in pd.password.chars() {
                if c == pd.character {
                    char_occurences += 1;
                }
            }

            if char_occurences >= pd.min && char_occurences <= pd.max {
                valid_passwords += 1;
            }
        }

        let ans = valid_passwords.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut valid_count = 0;

        for pd in &self.data {
            let password_chars = pd.password.chars().collect::<Vec<char>>();
            let mut valid_password = false;

            if password_chars[pd.min as usize - 1] == pd.character
                && password_chars[pd.max as usize - 1] == pd.character
            {
                valid_password = false;
            } else if password_chars[pd.min as usize - 1] == pd.character
                || password_chars[pd.max as usize - 1] == pd.character
            {
                valid_password = true;
            }

            if valid_password {
                valid_count += 1;
            }
        }

        let ans = valid_count.to_string();

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
        let expected_q1 = String::from("2");

        let mut day2 = Day2::new();

        day2.init("inputs/2020/2a.txt")
            .expect("error trying to init day2");

        let q1 = day2.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("1");

        let mut day2 = Day2::new();

        day2.init("inputs/2020/2a.txt")
            .expect("error trying to init day2");

        let q2 = day2.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
