use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use regex::{Captures, Regex};
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

pub fn confirm(q: &str, default: bool) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(q)
        .default(default)
        .interact()
        .expect("error trying to confirm")
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
        .expect("expected a valid file at the specified path but found none")
        .trim()
        .to_string();
}

pub struct RegexCaptures<'a> {
    captures: Captures<'a>,
}

impl<'a> RegexCaptures<'a> {
    pub fn new(captures: Captures<'a>) -> Self {
        Self { captures }
    }

    pub fn get_name(&self, name: &str) -> String {
        self.captures
            .name(name)
            .expect("error trying to get captures")
            .as_str()
            .to_string()
    }

    pub fn get_name_usize(&self, name: &str) -> usize {
        self.captures
            .name(name)
            .expect("error trying to get captures")
            .as_str()
            .parse()
            .expect("error trying to convert string to usize")
    }
}

pub struct RegexParser {
    pattern: Regex,
}

impl RegexParser {
    pub fn new(pattern: &str) -> Self {
        let pattern = Regex::new(pattern).expect(
            format!("error initializing regex parser with the given pattern - {pattern}").as_str(),
        );
        Self { pattern }
    }

    pub fn parse<'a>(&self, txt: &'a str) -> RegexCaptures<'a> {
        let captures = self.pattern.captures(txt).expect("error fetching captures");

        RegexCaptures::new(captures)
    }
}
