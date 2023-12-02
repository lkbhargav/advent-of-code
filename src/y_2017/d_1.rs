use crate::prelude::*;

pub struct Day1 {
    inp: String,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2017/1.txt";
// const TEST_FILE_NAME: &str = "inputs/2017/1a.txt";

impl Questions for Day1 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.inp = contents.lines().filter(|f| !f.is_empty()).collect();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut tail_ends = ('a', 'z');
        let mut prev: char = 'a';
        let mut to_sum = 0;

        for (index, c) in self.inp.chars().enumerate() {
            if index == 0 {
                tail_ends.0 = c;
            } else if index == self.inp.len() - 1 {
                tail_ends.1 = c;
            }

            if prev != 'a' && prev == c {
                to_sum += prev.to_digit(10).expect("error parsing char to digit");
            }

            prev = c;
        }

        if tail_ends.0 == tail_ends.1 {
            to_sum += tail_ends
                .0
                .to_digit(10)
                .expect("error parsing char to digit");
        }

        let ans = to_sum.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let inp_len = self.inp.len();
        let inp_half = inp_len / 2;
        let mut to_sum = 0;

        let chars: Vec<&str> = self.inp.split("").filter(|c| !c.is_empty()).collect();

        for (index, c) in chars.iter().enumerate() {
            let cmp_idx = (index + inp_half) % inp_len;

            if *c == chars[cmp_idx] {
                to_sum += c.parse::<u32>().expect("error parsing char to digit");
            }
        }

        let ans = to_sum.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 { inp: String::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("4");

        let mut day1 = Day1::new();

        day1.init("inputs/2017/1a.txt")
            .expect("error trying to init day1");

        let q1 = day1.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q1b_works() {
        let expected_q1 = String::from("0");

        let mut day1 = Day1::new();

        day1.init("inputs/2017/1b.txt")
            .expect("error trying to init day1");

        let q1 = day1.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q1c_works() {
        let expected_q1 = String::from("9");

        let mut day1 = Day1::new();

        day1.init("inputs/2017/1c.txt")
            .expect("error trying to init day1");

        let q1 = day1.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q1d_works() {
        let expected_q1 = String::from("3");

        let mut day1 = Day1::new();

        day1.init("inputs/2017/1d.txt")
            .expect("error trying to init day1");

        let q1 = day1.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("6");

        let mut day1 = Day1::new();

        day1.init("inputs/2017/1e.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }

    #[test]
    fn q2f_works() {
        let expected_q2 = String::from("0");

        let mut day1 = Day1::new();

        day1.init("inputs/2017/1f.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }

    #[test]
    fn q2g_works() {
        let expected_q2 = String::from("4");

        let mut day1 = Day1::new();

        day1.init("inputs/2017/1g.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }

    #[test]
    fn q2h_works() {
        let expected_q2 = String::from("12");

        let mut day1 = Day1::new();

        day1.init("inputs/2017/1h.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }

    #[test]
    fn q2i_works() {
        let expected_q2 = String::from("4");

        let mut day1 = Day1::new();

        day1.init("inputs/2017/1i.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
