use std::error::Error;

pub trait Questions {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn Error>>;
    fn question_one(&mut self) -> Result<String, Box<dyn Error>>;
    fn question_two(&mut self) -> Result<String, Box<dyn Error>>;
}
