use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Instruction {
    count: u16,
    from: u16,
    to: u16,
}

impl Instruction {
    pub fn new(count: u16, from: u16, to: u16) -> Self {
        Self { count, from, to }
    }
}

pub struct Day5 {
    data: Vec<Vec<char>>,
    inp: Vec<Instruction>,
    is_test: bool,
    test_file: String,
    prod_file: String,
}

// uncomment the following line incase you want to get the file name to reintialize
const FILE_NAME: &str = "inputs/2022/5.txt";

impl Questions for Day5 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let vals = contents.split("\n\n").collect::<Vec<&str>>();
        let initial_values = vals[0];
        let instructions = vals[1];

        for line in initial_values.lines() {
            let mut vec = vec![];

            let vals = regex_parser(r#"\d -> (?P<vals>[A-Z]+)"#, line).get_name("vals");

            for c in vals.chars() {
                vec.push(c);
            }

            self.data.push(vec);
        }

        for line in instructions.lines() {
            let repa = regex_parser(
                r#"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)"#,
                line,
            );

            let count: u16 = repa.get_name_usize("count") as u16;
            let from: u16 = repa.get_name_usize("from") as u16 - 1;
            let to: u16 = repa.get_name_usize("to") as u16 - 1;

            self.inp.push(Instruction::new(count, from, to));
        }

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        for instruction in self.inp.clone() {
            self.perform_instruction(&instruction);
        }

        let ans = self.get_top_values();

        println!("\nAnswer to first question is {}\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.data.clear();
        self.inp.clear();

        let mut file_name = self.prod_file.clone();

        if self.is_test {
            file_name = self.test_file.clone();
        }

        self.init(&file_name)
            .expect("error reinitializing the data set");

        for instruction in self.inp.clone() {
            self.perform_instruction_q2(&instruction);
        }

        let ans = self.get_top_values();

        println!("\nAnswer to second question is {}\n", ans.green());

        Ok(ans)
    }
}

impl Day5 {
    pub fn new(is_test: bool) -> Day5 {
        Day5 {
            data: vec![],
            inp: vec![],
            is_test,
            test_file: String::from("inputs/2022/5a.txt"),
            prod_file: String::from(FILE_NAME),
        }
    }

    pub fn perform_instruction(&mut self, instruction: &Instruction) {
        let count = instruction.count;
        let from_index = instruction.from as usize;
        let to_index = instruction.to as usize;

        for _ in 0..count {
            let val = self.data[from_index].pop().unwrap();
            self.data[to_index].push(val);
        }
    }

    pub fn perform_instruction_q2(&mut self, instruction: &Instruction) {
        let count = instruction.count;
        let from_index = instruction.from as usize;
        let to_index = instruction.to as usize;

        let mut tmp = vec![];

        for _ in 0..count {
            let val = self.data[from_index].pop().unwrap();
            tmp.push(val);
        }

        for _ in 0..count {
            let val = tmp.pop().unwrap();
            self.data[to_index].push(val);
        }
    }

    pub fn get_top_values(&self) -> String {
        let mut response = String::new();

        for v in &self.data {
            response.push(v[v.len() - 1]);
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("CMZ");

        let mut day5 = Day5::new(true);

        day5.init("inputs/2022/5a.txt")
            .expect("error trying to init day5");

        let q1 = day5.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("MCD");

        let mut day5 = Day5::new(true);

        day5.init("inputs/2022/5a.txt")
            .expect("error trying to init day5");

        let q2 = day5.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }
}
