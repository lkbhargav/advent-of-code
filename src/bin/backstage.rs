use advent_of_code::prelude::*;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() {
    let year = prompt_with_validation("Enter the year?", "2015", |i: &str| -> Option<&str> {
        if i.len() != 4 {
            return Some("Invalid year entered!");
        }

        if !i.chars().all(char::is_numeric) {
            return Some("Year should be in numbers only!");
        }

        None
    });

    let day = prompt_with_validation("Enter the day?", "21", |i: &str| -> Option<&str> {
        if i.is_empty() {
            return Some("Age cannot be empty!");
        }

        if !i.chars().all(char::is_numeric) {
            return Some("Age should be in numbers only");
        }

        None
    });

    /* Things to do:
        1. Check if a file already exists
            a. If exists, then throw a warning that the file already exists.
            b. If not exists, then create a file and required folders and updates to the file if it's a new year completely.
    */

    let current_dir = &get_current_dir();

    let new_year_dir_name = format!("y_{year}");

    let file_path = format!("{}/src/{new_year_dir_name}", current_dir);

    let does_year_exists = Path::new(&file_path).exists();

    if !does_year_exists {
        fs::create_dir(&file_path).expect("error trying to create a directory with the given year");

        let file_path = &format!("{file_path}/mod.rs");
        File::create(file_path).expect("error trying to create a file");
    }

    let file_path = &format!("{file_path}/d_{day}.rs");

    if !Path::new(&file_path).exists() {
        File::create(file_path).expect("error trying to create a file");
    }

    // at this point a directory is either created or already exists

    // append the `pub mod y_2016` line to the `lib.rs` file

    let text_to_append = &format!("pub mod {new_year_dir_name};");
    let lib_file = &format!("{current_dir}/src/lib.rs");
    if !find_the_text_in_file(lib_file, text_to_append) {
        append_to_file(lib_file, text_to_append);
    }

    let text_to_append = &format!("pub mod d_{day};");
    let mod_file = &format!("{current_dir}/src/y_{year}/mod.rs");
    if !find_the_text_in_file(mod_file, text_to_append) {
        append_to_file(mod_file, text_to_append);
    }

    let text_to_append = &get_template_solution(&day, &year);
    let day_file = &format!("{current_dir}/src/y_{year}/d_{day}.rs");
    if is_the_file_empty(day_file) {
        append_to_file(day_file, text_to_append);
    }

    get_test_data(
        &year,
        &day,
        "Do you want to input `test` data?",
        "Enter the `test` data?",
        true,
        true,
    );

    get_test_data(
        &year,
        &day,
        "Do you want to input `real` data?",
        "Enter the `real` data?",
        true,
        false,
    );
}

fn get_current_dir() -> String {
    let current_dir = env::current_dir().expect("error trying to fetch the current directory");
    let current_dir = current_dir
        .to_str()
        .expect("error trying to get convert the directory path to string");
    current_dir.to_string()
}

fn append_to_file(file_name: &str, text_to_append: &str) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(file_name)
        .expect(&format!(
            "error trying to open the given file {file_name} to append"
        ));

    writeln!(file, "{}", text_to_append).expect(&format!(
        "error trying to append the text to the given file"
    ));
}

fn find_the_text_in_file(file_name: &str, text_to_search: &str) -> bool {
    let contents =
        fs::read_to_string(file_name).expect("error trying to read contents of the given file");

    for line in contents.lines() {
        if line.eq(text_to_search) {
            return true;
        }
    }

    false
}

fn is_the_file_empty(file_name: &str) -> bool {
    let contents =
        fs::read_to_string(file_name).expect(&format!("error trying to open the file {file_name}"));
    contents.lines().count() == 0
}

#[inline(always)]
fn get_test_data(
    year: &str,
    day: &str,
    confirmation_prompt_question: &str,
    prompt_question: &str,
    default_confirmation_prompt: bool,
    test_data: bool,
) {
    let test_data_suffix = if test_data { "a" } else { "" };

    let file_path = format!("inputs/{year}/{day}{test_data_suffix}.txt",);

    if !Path::new(&file_path).exists() {
        let test_data_confirmation =
            confirm(confirmation_prompt_question, default_confirmation_prompt);

        if test_data_confirmation {
            // Prompt for sample input
            let test_contents = prompt(prompt_question, "");

            if !test_contents.is_empty() {
                let file_path = format!("inputs/{}", year);
                if !Path::new(&file_path).exists() {
                    fs::create_dir_all(&file_path)
                        .expect("something went wrong creating a directory");
                }

                let file_path = format!("{file_path}/{day}{test_data_suffix}.txt");
                File::create(&file_path).expect("error trying to create a file");
                append_to_file(&file_path, &test_contents);
            }
        }
    }
}

fn get_template_solution(day: &str, year: &str) -> String {
    format!(
        r#"
use crate::prelude::*;

pub struct Day{day} {{}}

impl Questions for Day{day} {{
    fn init(&mut self, file: &str) -> Result<(), Box<dyn std::error::Error>> {{
        let contents = read_from_file(file);

        // TODO: file parsing logic goes here

        Ok(())
    }}

    fn question_one(&mut self) -> Result<String, Box<dyn std::error::Error>> {{
        // TODO: your logic goes in here...

        let ans = "na".to_string();

        println!("\nAnswer to first question is {{}}\n", ans.green());

        Ok(ans)
    }}

    fn question_two(&mut self) -> Result<String, Box<dyn std::error::Error>> {{
        // TODO: your logic goes in here...

        let ans = "na".to_string();

        println!("\nAnswer to second question is {{}}\n", ans.green());

        Ok(ans)
    }}
}}

impl Day{day} {{
    pub fn new() -> Day{day} {{
        Day{day} {{}}
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn q1_works() {{
        let expected_q1 = String::from("");

        let mut day{day} = Day{day}::new();

        day{day}.init("inputs/{year}/{day}a.txt")
            .expect("error trying to init day{day}");

        let q1 = day{day}.question_one().unwrap();

        assert_eq!(expected_q1, q1);
    }}

    #[test]
    fn q2_works() {{
        let expected_q2 = String::from("");
        
        let mut day{day} = Day{day}::new();

        day{day}.init("inputs/{year}/{day}a.txt")
            .expect("error trying to init day{day}");

        let q2 = day{day}.question_two().unwrap();

        assert_eq!(expected_q2, q2);
    }}
}}
        "#
    )
}
