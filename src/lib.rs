pub mod traits;
pub mod types;
pub mod util;
pub mod y_2015;
pub mod y_2016;
pub mod y_2017;
pub mod y_2022;

pub mod prelude {
    pub use crate::traits::{Number, Questions};
    pub use crate::types::QuestionNumber;
    pub use crate::util::{
        confirm, prompt, prompt_select, prompt_with_validation, read_from_file, RegexParser,
    };
    pub use colored::Colorize;
}
