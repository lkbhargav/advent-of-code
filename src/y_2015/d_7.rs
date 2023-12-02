use crate::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    AND,
    OR,
    NOT,
    LSHIFT,
    RSHIFT,
    NOOP,
}

impl Operation {
    pub fn operate(&self, inp: Vec<usize>) -> std::result::Result<usize, String> {
        let validation_err = Err("invalid number of arguments passed".to_string());
        match self {
            Operation::AND => {
                if inp.len() != 2 {
                    validation_err
                } else {
                    Ok(inp[0] & inp[1])
                }
            }
            Operation::OR => {
                if inp.len() != 2 {
                    validation_err
                } else {
                    Ok(inp[0] | inp[1])
                }
            }
            Operation::NOT => {
                if inp.len() != 1 {
                    validation_err
                } else {
                    Ok(65535 - inp[0])
                }
            }
            Operation::LSHIFT => {
                if inp.len() != 2 {
                    validation_err
                } else {
                    Ok(inp[0] << inp[1])
                }
            }
            Operation::RSHIFT => {
                if inp.len() != 2 {
                    validation_err
                } else {
                    Ok(inp[0] >> inp[1])
                }
            }
            Operation::NOOP => Ok(inp[0]),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    inputs: Vec<String>,
    op: Operation,
}

impl Instruction {
    pub fn new(input: &str) -> Instruction {
        let mut op = Operation::NOOP;
        let mut inputs = vec![];

        if input.starts_with("NOT") {
            op = Operation::NOT;
            let val = Instruction::fetch_a_val(input, "NOT");
            inputs.push(val.to_string());
        } else if input.contains("AND") {
            op = Operation::AND;
            let val = Instruction::fetch_two_vals(input, "AND");
            inputs.push(val.0.to_string());
            inputs.push(val.1.to_string());
        } else if input.contains("OR") {
            op = Operation::OR;
            let val = Instruction::fetch_two_vals(input, "OR");
            inputs.push(val.0.to_string());
            inputs.push(val.1.to_string());
        } else if input.contains("LSHIFT") {
            op = Operation::LSHIFT;
            let val = Instruction::fetch_two_vals(input, "LSHIFT");
            inputs.push(val.0.to_string());
            inputs.push(val.1.to_string());
        } else if input.contains("RSHIFT") {
            op = Operation::RSHIFT;
            let val = Instruction::fetch_two_vals(input, "RSHIFT");
            inputs.push(val.0.to_string());
            inputs.push(val.1.to_string());
        } else {
            inputs.push(input.trim().to_string());
        }

        Instruction { inputs, op }
    }

    pub fn fetch_a_val<'a>(input: &'a str, split_by: &str) -> &'a str {
        input
            .split(split_by)
            .filter(|v| !v.is_empty())
            .map(|v| v)
            .take(1)
            .collect::<Vec<&str>>()[0]
            .trim()
    }

    pub fn fetch_two_vals<'a>(input: &'a str, split_by: &str) -> (&'a str, &'a str) {
        let val = input
            .split(split_by)
            .filter(|v| !v.is_empty())
            .map(|v| v.trim())
            .collect::<Vec<&str>>();

        (val[0], val[1])
    }
}

pub struct Day7 {
    data: HashMap<String, Instruction>,
    map: HashMap<String, usize>,
}

impl Questions for Day7 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let regex_parser = RegexParser::new(r#"(?P<input>[a-zA-Z0-9| ]+) -> (?P<output>\w+)"#);

        contents
            .lines()
            .filter(|f| !f.is_empty())
            .for_each(|instruction| {
                let captures = regex_parser.parse(instruction);

                let input = captures.get_name("input");
                let output = captures.get_name("output");

                let instruction = Instruction::new(&input);
                if instruction.op == Operation::NOOP {
                    let inp = input.trim().parse::<usize>();
                    if inp.is_ok() {
                        self.map.insert(output.to_string(), inp.unwrap());
                    } else {
                        self.data.insert(output.to_string(), instruction);
                    }
                } else {
                    self.data.insert(output.to_string(), instruction);
                }
            });

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        for (key, instruction) in self.data.clone() {
            self.find(&instruction, &key);
        }

        let ans = self
            .map
            .get("a")
            .expect("expected the key `a` but found none");

        let ans = ans.to_string();

        println!("\nAnswer to 1st question: {}\n", ans.green().bold());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let a = *self.map.get("a").unwrap();

        self.map.clear();
        self.data.clear();

        self.init("inputs/2015/7.txt")
            .expect("error while reinitializing");

        self.map.insert("b".to_string(), a);

        for (key, instruction) in self.data.clone() {
            self.find(&instruction, &key);
        }

        let ans = self
            .map
            .get("a")
            .expect("expected the key `a` but found none");

        let ans = ans.to_string();

        println!("\nAnswer to 2nd question: {}\n", ans.green().bold());

        Ok(ans)
    }
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 {
            data: HashMap::new(),
            map: HashMap::new(),
        }
    }

    pub fn find(&mut self, v: &Instruction, key: &str) -> usize {
        let mut tmp = vec![];
        for (idx, inp) in v.inputs.clone().iter().enumerate() {
            if (v.op == Operation::LSHIFT || v.op == Operation::RSHIFT) && idx == 1 {
                let val = inp
                    .parse()
                    .expect("error parsing string to int in RSHIFT and LSHIFT");
                tmp.push(val);
                continue;
            }
            let val = &self.value_of(&inp);
            tmp.push(*val);
        }
        let res = (*v).op.operate(tmp).expect(
            format!("2nd place expected a valid response for the operation but got this error")
                .as_str(),
        );

        // updating the map with the computed values
        self.map.insert(key.to_string(), res);

        res
    }

    pub fn value_of(&mut self, key: &str) -> usize {
        match self.map.get(key) {
            None => match self.data.clone().get(key) {
                None => key
                    .parse::<usize>()
                    .expect(format!("error trying to parse string to int {key}").as_str()),
                Some(v) => self.find(v, key),
            },
            Some(v) => *v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("65412");

        let mut day7 = Day7::new();

        day7.init("inputs/2015/7a.txt")
            .expect("error trying to init day7");

        let q1 = day7.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    // we don't have test data for part 2, so commenting out the whole test
    // #[test]
    // fn q2_works() {
    //     let expected_q2 = String::from("");

    //     let mut day7 = Day7::new();

    //     day7.init("inputs/2015/7a.txt")
    //         .expect("error trying to init day7");

    //     let q2 = day7.question_two().unwrap();

    //     assert_eq!(expected_q2, q2);
    // }
}
