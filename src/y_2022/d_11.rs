use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum Operation {
    Sum,
    Mul,
}

impl Operation {
    pub fn new(op: &str) -> Operation {
        match op {
            "+" => Operation::Sum,
            "*" => Operation::Mul,
            _ => panic!("invalid operation passed"),
        }
    }

    pub fn operate(&self, old: usize, value: usize) -> usize {
        match self {
            Operation::Mul => old * value,
            Operation::Sum => old + value,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(usize),
    Old,
}

impl Value {
    pub fn new(v: &str) -> Self {
        match v {
            "old" => Value::Old,
            _ => Value::Number(v.to_usize("Value conversion")),
        }
    }

    pub fn fetch(&self, old_val: usize) -> usize {
        match *self {
            Self::Old => old_val,
            Self::Number(n) => n,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonkeyOps {
    operation: (Operation, Value),
    divisible_by: usize,
    true_condition: usize,
    false_condition: usize,
}

impl MonkeyOps {
    pub fn new(
        operation: (Operation, Value),
        divisible_by: usize,
        true_condition: usize,
        false_condition: usize,
    ) -> Self {
        Self {
            operation,
            divisible_by,
            true_condition,
            false_condition,
        }
    }

    pub fn perform_ops<F: Fn(usize) -> usize>(
        &self,
        values: Vec<usize>,
        bored: F,
    ) -> (Vec<(usize, usize)>, usize) {
        let mut to_return = vec![];

        for val in values.clone() {
            let value_part = self.operation.1.fetch(val);
            let new_val = self.operation.0.operate(val, value_part);

            let after_bored = bored(new_val);

            if after_bored % self.divisible_by == 0 {
                to_return.push((after_bored, self.true_condition));
            } else {
                to_return.push((after_bored, self.false_condition));
            }
        }

        (to_return, values.len() as usize)
    }
}

pub struct Day11 {
    monkey_ops: Vec<MonkeyOps>,
    monkeys: Vec<Vec<usize>>,
    is_test: bool,
}

// uncomment the following line incase you want to get the file name to reintialize
const FILE_NAME: &str = "inputs/2022/11.txt";
const TEST_FILE_NAME: &str = "inputs/2022/11a.txt";

impl Questions for Day11 {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let contents = read_from_file(file);

        let regex_parser_line_2 =
            RegexParser::new(r#"old (?P<operation>\+|\*) (?P<value>\d+|\w+)"#);
        let regex_parser_line_3 = RegexParser::new(r#"by (?P<divisible_by>\d+)"#);
        let regex_parser_line_n = RegexParser::new(
            r#"If (?P<condition>true|false): throw to monkey (?P<monkey_index>\d+)"#,
        );

        let mut monkeys = vec![];
        let mut monkey_ops = vec![];

        contents
            .split("\n\n")
            .filter(|f| !f.is_empty())
            .for_each(|v| {
                let mut operation = (Operation::Sum, Value::Old);
                let mut divisible_by = 0;
                let mut true_condition = 0;
                let mut false_condition = 0;

                for (line_num, txt) in v.lines().enumerate() {
                    if line_num == 0 {
                        monkeys.push(vec![]);
                    }

                    if line_num == 1 {
                        let data = txt.trim().split(": ").collect::<Vec<&str>>()[1]
                            .split(",")
                            .map(|v| v.trim().to_usize("numbers"))
                            .collect::<Vec<usize>>();

                        let monkeys_len = monkeys.len();
                        monkeys[monkeys_len - 1] = data;
                    }

                    if line_num == 2 {
                        let captures = regex_parser_line_2.parse(txt);
                        let op = captures.get_name("operation");
                        let value = captures.get_name("value");

                        operation = (Operation::new(&op), Value::new(&value));
                    }

                    if line_num == 3 {
                        let captures = regex_parser_line_3.parse(txt);
                        let dby = captures.get_name("divisible_by");

                        divisible_by = dby.to_usize("divisible_by");
                    }

                    if line_num == 4 {
                        let captures = regex_parser_line_n.parse(txt);
                        let midx = captures.get_name("monkey_index");

                        true_condition = midx.to_usize("true_condition");
                    }

                    if line_num == 5 {
                        let captures = regex_parser_line_n.parse(txt);
                        let midx = captures.get_name("monkey_index");

                        false_condition = midx.to_usize("false_condition");
                    }
                }

                monkey_ops.push(MonkeyOps::new(
                    operation,
                    divisible_by,
                    true_condition,
                    false_condition,
                ))
            });

        self.monkey_ops = monkey_ops;
        self.monkeys = monkeys;

        Ok(())
    }

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut monkeys_processed: Vec<usize> = Vec::with_capacity(self.monkeys.len());

        for _ in 0..self.monkeys.len() {
            monkeys_processed.push(0);
        }

        let bored = Day11::bored;

        for _ in 0..20 {
            for (monkey, ops) in self.monkey_ops.iter().enumerate() {
                let results = ops.perform_ops(self.monkeys[monkey].clone(), bored.clone());

                for (new_value, new_monkey) in results.0 {
                    self.monkeys[new_monkey as usize].push(new_value);
                }

                monkeys_processed[monkey] += results.1;

                self.monkeys[monkey].clear();
            }
        }

        monkeys_processed.sort_by(|a, b| b.cmp(a));

        let ans = monkeys_processed.into_iter().take(2).product::<usize>();

        let ans = ans.to_string();

        println!("\nAnswer to first question is {}\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        // reinitializing here
        if self.is_test {
            self.init(TEST_FILE_NAME)
                .expect("error trying to reinitialize the values");
        } else {
            self.init(FILE_NAME)
                .expect("error trying to reinitialize the values");
        }

        let mut monkeys_processed: Vec<usize> = Vec::with_capacity(self.monkeys.len());

        for _ in 0..self.monkeys.len() {
            monkeys_processed.push(0);
        }

        let val = self
            .monkey_ops
            .iter()
            .map(|a| a.divisible_by)
            .product::<usize>();

        let bored = |v: usize| -> usize { v % (val as usize) };

        for _ in 0..10_000 {
            for (monkey, ops) in self.monkey_ops.iter().enumerate() {
                let results = ops.perform_ops(self.monkeys[monkey].clone(), bored.clone());

                for (new_value, new_monkey) in results.0 {
                    self.monkeys[new_monkey as usize].push(new_value);
                }

                monkeys_processed[monkey] += results.1;

                self.monkeys[monkey].clear();
            }
        }

        monkeys_processed.sort_by(|a, b| b.cmp(a));

        let ans = monkeys_processed.into_iter().take(2).product::<usize>();

        let ans = ans.to_string();

        println!("\nAnswer to second question is {}\n", ans.green());

        Ok(ans)
    }
}

impl Day11 {
    pub fn new(is_test: bool) -> Day11 {
        Day11 {
            monkey_ops: vec![],
            monkeys: vec![],
            is_test,
        }
    }

    pub fn bored(val: usize) -> usize {
        val / 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q1_works() {
        let expected_q1 = String::from("10605");

        let mut day11 = Day11::new(true);

        day11
            .init("inputs/2022/11a.txt")
            .expect("error trying to init day11");

        let q1 = day11.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }

    #[test]
    fn q2_works() {
        let expected_q2 = String::from("2713310158");

        let mut day11 = Day11::new(true);

        day11
            .init("inputs/2022/11a.txt")
            .expect("error trying to init day11");

        let q2 = day11.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }

    #[test]
    fn test_bored() {
        assert_eq!(5, Day11::bored(16));
        assert_eq!(5, Day11::bored(15));
        assert_eq!(5, Day11::bored(17));
    }
}
