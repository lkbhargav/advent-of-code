use std::collections::HashSet;

use crate::prelude::*;

#[derive(Debug, Default)]
struct Store {
    supernet_string: Vec<String>,
    hypernet_strings: Vec<String>,
}

impl Store {
    fn is_valid_ip7_q1(&self) -> bool {
        for is in &self.hypernet_strings {
            if Store::contains_a_4_char_palindrome_substring(is) {
                return false;
            }
        }

        for vs in &self.supernet_string {
            if Store::contains_a_4_char_palindrome_substring(vs) {
                return true;
            }
        }

        false
    }

    fn is_valid_ip7_q2(&self) -> bool {
        let mut supernet_set = HashSet::new();
        let mut hypernet_set = HashSet::new();

        for str in &self.hypernet_strings {
            Store::contains_a_3_char_palindrome_substring(str, &mut hypernet_set);
        }

        for str in &self.supernet_string {
            Store::contains_a_3_char_palindrome_substring(str, &mut supernet_set);
        }

        if hypernet_set.len() == 0 || supernet_set.len() == 0 {
            return false;
        }

        for k in hypernet_set {
            if supernet_set.contains(&(k.1, k.0)) {
                return true;
            }
        }

        false
    }

    fn contains_a_3_char_palindrome_substring(inp: &str, set: &mut HashSet<(char, char)>) {
        if inp.len() < 3 {
            return;
        }

        let inp: Vec<char> = inp.chars().collect();

        let mut l = 0;
        let mut r = 2;

        for _ in 2..inp.len() {
            if inp[l] == inp[r] {
                set.insert((inp[l + 1], inp[r]));
            }

            l += 1;
            r += 1;
        }

        return;
    }

    fn contains_a_4_char_palindrome_substring(inp: &str) -> bool {
        if inp.len() < 4 {
            return false;
        }

        let inp: Vec<char> = inp.chars().collect();

        let mut l = 1;
        let mut r = 2;

        loop {
            let mut group_count = 0;

            let mut tl = l;
            let mut tr = r;

            let mut prev_char = ' ' as char;

            // since we are only concerned about 4 characters (subword)
            for _ in 0..2 {
                if inp[tl] == inp[tr] && inp[tl] != prev_char {
                    group_count += 1;
                }

                if group_count == 2 {
                    return true;
                }

                if tl == 0 || tr + 1 == inp.len() {
                    break;
                }

                prev_char = inp[tl];

                tl -= 1;
                tr += 1;
            }

            l += 1;
            r += 1;

            if r >= inp.len() {
                break;
            }
        }

        false
    }
}

pub struct Day7 {
    data: Vec<Store>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2016/7.txt";
// const TEST_FILE_NAME: &str = "inputs/2016/7a.txt";

impl Questions for Day7 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        contents.lines().filter(|f| !f.is_empty()).for_each(|line| {
            let mut str = vec![];
            let mut is_valid_str = true;
            let mut store = Store::default();

            for ch in line.chars() {
                match ch {
                    '[' => {
                        is_valid_str = false;
                        store.supernet_string.push(str.join(""));
                        str.clear();
                    }
                    ']' => {
                        is_valid_str = true;
                        store.hypernet_strings.push(str.join(""));
                        str.clear();
                    }
                    _ => str.push(ch.to_string()),
                }
            }

            if is_valid_str {
                store.supernet_string.push(str.join(""));
            } else {
                store.hypernet_strings.push(str.join(""));
            }

            self.data.push(store);
        });

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut count = 0;

        for s in &self.data {
            if s.is_valid_ip7_q1() {
                count += 1;
            }
        }

        let ans = count.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut count = 0;

        for s in &self.data {
            if s.is_valid_ip7_q2() {
                count += 1;
            }
        }

        let ans = count.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 { data: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("2");

        let mut day7 = Day7::new();

        day7.init("inputs/2016/7a.txt")
            .expect("error trying to init day7");

        let q1 = day7.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("3");

        let mut day7 = Day7::new();

        day7.init("inputs/2016/7b.txt")
            .expect("error trying to init day7");

        let q2 = day7.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
