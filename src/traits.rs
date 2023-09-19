use std::{borrow::Cow, error::Error};

pub trait Questions {
    fn init(&mut self, file: &str) -> Result<(), Box<dyn Error>>;
    fn question_one(&mut self) -> Result<String, Box<dyn Error>>;
    fn question_two(&mut self) -> Result<String, Box<dyn Error>>;
}

pub trait Number {
    fn to_usize(&self, message: &str) -> usize;
    fn to_u128(&self, message: &str) -> u128;
    fn to_u64(&self, message: &str) -> u64;
    fn to_u32(&self, message: &str) -> u32;
    fn to_u16(&self, message: &str) -> u16;
    fn to_i32(&self, message: &str) -> i32;
    fn to_isize(&self, message: &str) -> isize;
}

macro_rules! impl_number {
    ($type: ty) => {
        impl Number for $type {
            fn to_usize(&self, message: &str) -> usize {
                self.parse::<usize>().expect(
                    format!("error converting string to usize. Message: {}", message).as_str(),
                )
            }

            fn to_u128(&self, message: &str) -> u128 {
                self.parse::<u128>().expect(
                    format!("error converting string to u128. Message: {}", message).as_str(),
                )
            }

            fn to_u64(&self, message: &str) -> u64 {
                self.parse::<u64>().expect(
                    format!("error converting string to u64. Message: {}", message).as_str(),
                )
            }

            fn to_u32(&self, message: &str) -> u32 {
                self.parse::<u32>().expect(
                    format!("error converting string to u32. Message: {}", message).as_str(),
                )
            }

            fn to_u16(&self, message: &str) -> u16 {
                self.parse::<u16>().expect(
                    format!("error converting string to u16. Message: {}", message).as_str(),
                )
            }

            fn to_i32(&self, message: &str) -> i32 {
                self.parse::<i32>().expect(
                    format!("error converting string to i32. Message: {}", message).as_str(),
                )
            }

            fn to_isize(&self, message: &str) -> isize {
                self.parse::<isize>().expect(
                    format!("error converting string to isize. Message: {}", message).as_str(),
                )
            }
        }
    };
}

impl_number!(&str);
impl_number!(Cow<'_, str>);
impl_number!(String);
