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

    println!("File Path: {}", file_path);

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

    let text_to_append = &get_template_solution(&day);
    let day_file = &format!("{current_dir}/src/y_{year}/d_{day}.rs");
    if is_the_file_empty(day_file) {
        append_to_file(day_file, text_to_append);
    }
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

fn get_template_solution(problem_number: &str) -> String {
    format!(
        r#"
use crate::prelude::*;

pub struct Day{problem_number} {{}}

impl Questions for Day{problem_number} {{
    fn question_one(&self) -> Result<(), Box<dyn std::error::Error>> {{
        println!("Question 1!");
        Ok(())
    }}

    fn question_two(&self) -> Result<(), Box<dyn std::error::Error>> {{
        println!("Question 2!");
        Ok(())
    }}
}}

impl Day{problem_number} {{
    pub fn new() -> Day{problem_number} {{
        return Day{problem_number} {{}};
    }}
}}            
        "#
    )
}
