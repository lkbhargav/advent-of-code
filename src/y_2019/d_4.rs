use crate::prelude::*;

pub struct Day4 {
    min: u32,
    max: u32,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2019/4.txt";
// const TEST_FILE_NAME: &str = "inputs/2019/4a.txt";

impl Questions for Day4 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let (min, max) = contents
            .trim()
            .split_once("-")
            .expect("error trying to split at -");

        self.min = min
            .parse()
            .expect("error trying to parse the inputs for min");
        self.max = max
            .parse()
            .expect("error trying to parse the inputs for max");

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut counter = 0;

        for num in self.min..self.max {
            if Day4::is_valid(num, false) {
                counter += 1;
            }
        }

        let ans = counter.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut counter = 0;

        for num in self.min..self.max {
            if Day4::is_valid(num, true) {
                counter += 1;
            }
        }

        let ans = counter.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 { min: 0, max: 0 }
    }

    fn is_valid(num: u32, is_q2: bool) -> bool {
        let mut found_a_pair = false;

        // found if the numbers are in ascending order
        let char_vec = num
            .to_string()
            .chars()
            .map(|c| c as u8)
            .collect::<Vec<u8>>();

        if char_vec.len() != 6 {
            return false;
        }

        let mut count = 0;

        loop {
            if count + 1 >= char_vec.len() {
                break;
            }

            if char_vec[count] > char_vec[count + 1] {
                return false;
            }

            if char_vec[count] == char_vec[count + 1] {
                if is_q2 {
                    let mut alike_counter = 2;

                    loop {
                        count += 1;

                        if count + 1 >= char_vec.len() {
                            count -= 1;
                            break;
                        }

                        if char_vec[count] == char_vec[count + 1] {
                            alike_counter += 1;
                        } else {
                            count -= 1;
                            break;
                        }
                    }

                    if alike_counter == 2 {
                        found_a_pair = true;
                    }
                } else {
                    found_a_pair = true;
                }
            }

            count += 1;
        }

        if !found_a_pair {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_works() {
        let expected_q1 = true;
        let res = Day4::is_valid(122456, false);
        assert_eq!(expected_q1, res);

        let expected_q1 = false;
        let res = Day4::is_valid(1533456, false);
        assert_eq!(expected_q1, res);

        let expected_q1 = false;
        let res = Day4::is_valid(153345, false);
        assert_eq!(expected_q1, res);
    }

    #[test]
    fn is_valid_works_q2() {
        let expected_q1 = false;
        let res = Day4::is_valid(122245, true);
        assert_eq!(expected_q1, res);

        let expected_q1 = true;
        let res = Day4::is_valid(122345, true);
        assert_eq!(expected_q1, res);

        let expected_q1 = false;
        let res = Day4::is_valid(153345, true);
        assert_eq!(expected_q1, res);

        let expected_q1 = false;
        let res = Day4::is_valid(123444, true);
        assert_eq!(expected_q1, res);

        let expected_q1 = true;
        let res = Day4::is_valid(112233, true);
        assert_eq!(expected_q1, res);

        let expected_q1 = true;
        let res = Day4::is_valid(111122, true);
        assert_eq!(expected_q1, res);

        let expected_q1 = false;
        let res = Day4::is_valid(111222, true);
        assert_eq!(expected_q1, res);
    }

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("1");

        let mut day4 = Day4::new();

        day4.init("inputs/2019/4a.txt")
            .expect("error trying to init day4");

        let q1 = day4.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("0");

        let mut day4 = Day4::new();

        day4.init("inputs/2019/4a.txt")
            .expect("error trying to init day4");

        let q2 = day4.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
