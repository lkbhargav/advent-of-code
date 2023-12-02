use crate::prelude::*;

pub struct Day1 {
    data: Vec<String>,
}

// uncomment the following line incase you want to get the file name to reintialize
// const FILE_NAME: &str = "inputs/2023/1.txt";
// const TEST_FILE_NAME: &str = "inputs/2023/1a.txt";

impl Questions for Day1 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let data = contents
            .lines()
            .filter(|f| !f.is_empty())
            .map(|v| v.to_string())
            .collect::<Vec<String>>();

        self.data = data;

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let sum = self.find_sum_of_all_nums(&|l: &str| l.to_string());

        let ans = sum.to_string();

        println!("\nAnswer to first question is {}\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let to_replace = vec![
            ("one", "o1e"),
            ("two", "t2o"),
            ("three", "t3e"),
            ("four", "f4r"),
            ("five", "f5e"),
            ("six", "s6x"),
            ("seven", "s7n"),
            ("eight", "e8e"),
            ("nine", "n9e"),
        ];

        let sum = self.find_sum_of_all_nums(&|l: &str| {
            let mut line = l.to_string();

            for tr in &to_replace {
                line = line.replace(tr.0, tr.1);
            }

            line
        });

        let ans = sum.to_string();

        println!("\nAnswer to second question is {}\n", ans.green());

        Ok(ans)
    }
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 { data: vec![] }
    }

    fn find_sum_of_all_nums(&self, line_processor: &dyn Fn(&str) -> String) -> usize {
        let mut sum = 0;

        let regex_pattern_first_val = RegexParser::new("(?P<val>\\d)");
        let regex_pattern_last_val = RegexParser::new(".*(?P<val>\\d)");

        for line in &self.data {
            let line = line_processor(line);

            let v = regex_pattern_first_val.parse(&line);
            let first_val = v.get_name_usize("val");

            let v = regex_pattern_last_val.parse(&line);
            let last_val = v.get_name_usize("val");

            let val = format!("{first_val}{last_val}")
                .parse::<usize>()
                .expect("couldn't parse");

            sum += val;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("142");

        let mut day1 = Day1::new();

        day1.init("inputs/2023/1a.txt")
            .expect("error trying to init day1");

        let q1 = day1.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("281");

        let mut day1 = Day1::new();

        day1.init("inputs/2023/1b.txt")
            .expect("error trying to init day1");

        let q2 = day1.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
