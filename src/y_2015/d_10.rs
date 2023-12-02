use crate::prelude::*;

pub struct Day10 {
    inp: Vec<char>,
    is_test: bool,
    iteration_count: usize,
    file_name: String,
    q2_iteration_count: usize,
}

// uncomment the following line incase you want to get the file name to reintialize
const FILE_NAME: &str = "inputs/2015/10.txt";

impl Questions for Day10 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        self.inp = contents.chars().collect::<Vec<char>>();

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        dbg!(self.is_test);

        self.run(self.iteration_count);

        let ans = self.inp.len().to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.inp.clear();
        self.init(self.file_name.clone().as_str())
            .expect("error trying to reinitialize the file");

        self.run(self.q2_iteration_count);

        let ans = self.inp.len().to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day10 {
    pub fn new(is_test: bool) -> Day10 {
        let mut iteration_count = 40;
        let mut file_name = FILE_NAME;

        if is_test {
            iteration_count = 5;
            file_name = "inputs/2015/10.txt"
        }

        Day10 {
            inp: vec![],
            is_test,
            iteration_count,
            file_name: file_name.to_string(),
            q2_iteration_count: 50,
        }
    }

    pub fn run(&mut self, iteration_count: usize) {
        for _ in 0..iteration_count {
            let mut tmp = vec![];
            let mut char_sequence = String::new();
            let mut prev_char = '-';

            for (idx, c) in self.inp.clone().iter().enumerate() {
                if self.inp.len() == 1 {
                    char_sequence = format!("{char_sequence}{}", *c);
                    break;
                }

                if idx > 0 {
                    if prev_char != *c {
                        tmp.extend(Day10::get_num_as_chars(char_sequence.len()));
                        tmp.push(char_sequence.chars().collect::<Vec<char>>()[0]);

                        char_sequence = String::new();
                    }
                }

                char_sequence = format!("{char_sequence}{}", *c);
                prev_char = *c;
            }

            // handling the last set
            tmp.extend(Day10::get_num_as_chars(char_sequence.len()));
            tmp.push(char_sequence.chars().collect::<Vec<char>>()[0]);

            self.inp = tmp;
        }
    }

    pub fn get_num_as_chars(num: usize) -> Vec<char> {
        let num = num.to_string();
        num.chars().collect::<Vec<char>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("6");

        let mut day10 = Day10::new(true);

        day10
            .init("inputs/2015/10a.txt")
            .expect("error trying to init day10");

        let q1 = day10.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    // Again, this test is not required since we just have to bump up the number of iterations for Q2 and everything else remains the same
    // #[test]
    // fn q2_works() {
    //     let expected_q2 = String::from("na");

    //     let mut day10 = Day10::new(true);

    //     day10
    //         .init("inputs/2015/10a.txt")
    //         .expect("error trying to init day10");

    //     let q2 = day10.question_two().unwrap();

    //     assert_eq!(expected_q2, q2);
    // }
}
