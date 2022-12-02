#[derive(PartialEq, Eq)]
pub enum QuestionNumber {
    One,
    Two,
}

impl QuestionNumber {
    pub fn parse(s: &str) -> Result<QuestionNumber, String> {
        match s {
            "1" | "one" => Ok(QuestionNumber::One),
            "2" | "two" => Ok(QuestionNumber::Two),
            _ => Err(String::from("invalid question number")),
        }
    }
}
