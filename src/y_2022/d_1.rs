use crate::prelude::*;

pub struct Day1 {
    inp: Vec<u32>,
}

impl Questions for Day1 {
    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = String::new();

        ans = (&self.inp.iter().max().unwrap()).to_string();

        println!("\nAnswer to first question is {}!\n", ans.green());

        Ok(ans)
    }

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let mut ans = String::new();

        ans = (&self.inp.iter().take(3).sum::<u32>()).to_string();

        println!("\nAnswer to second question is {}!\n", ans.green());

        Ok(ans)
    }
}

impl Day1 {
    pub fn new(file: &str) -> Day1 {
        let input = read_from_file(file);

        let mut elves_input = input
            .split("\n\n")
            .into_iter()
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|v| {
                v.lines()
                    .filter(|v| !v.is_empty())
                    .map(|v| {
                        v.parse::<u32>()
                            .expect("something went wrong converting string to int")
                    })
                    .sum::<u32>()
            })
            .collect::<Vec<u32>>();

        elves_input.sort_by(|a, b| b.partial_cmp(a).unwrap());

        return Day1 { inp: elves_input };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let expected_q1 = String::from("24000");
        let expected_q2 = String::from("45000");

        let mut day21 = Day1::new("inputs/2022/1a.txt");

        let q1 = day21.question_one().unwrap();
        let q2 = day21.question_two().unwrap();

        assert_eq!(expected_q1, q1);
        assert_eq!(expected_q2, q2);
    }
}
