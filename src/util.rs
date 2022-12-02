use colored::*;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::fs;

pub fn prompt_select<T>(q: &str, items: &Vec<T>) -> usize
where
    T: std::fmt::Display,
{
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(q)
        .default(0)
        .items(items)
        .interact()
        .expect("error trying to render a select")
}

pub fn prompt(q: &str, default: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(q)
        .default(default.to_string())
        .interact_text()
        .expect("error trying to get input")
}

pub fn prompt_with_validation<'a>(
    q: &str,
    default: &str,
    validator: fn(&str) -> Option<&str>,
) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .validate_with(|i: &String| -> Result<(), &str> {
            let val = validator(i);
            if val.is_none() {
                return Ok(());
            }
            println!("{}", val.unwrap().bright_red().on_bright_black());
            Err("validation failed")
        })
        .with_prompt(q)
        .default(default.to_string())
        .interact_text()
        .expect("error trying to get input")
}

pub fn read_from_file(path: &str) -> String {
    return fs::read_to_string(path)
        .expect("expected a valid file at the specified path but found none");
}
