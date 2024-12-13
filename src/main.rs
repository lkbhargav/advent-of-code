use std::time::Instant;

use advent_of_code::{
    prelude::*,
    y_2015::{
        d_1::Day1 as y15d1, d_10::Day10 as y15d10, d_11::Day11 as y15d11, d_12::Day12 as y15d12,
        d_2::Day2 as y15d2, d_3::Day3 as y15d3, d_4::Day4 as y15d4, d_5::Day5 as y15d5,
        d_6::Day6 as y15d6, d_7::Day7 as y15d7, d_8::Day8 as y15d8, d_9::Day9 as y15d9,
    },
    y_2016::{
        d_1::Day1 as y16d1, d_2::Day2 as y16d2, d_3::Day3 as y16d3, d_4::Day4 as y16d4,
        d_5::Day5 as y16d5, d_6::Day6 as y16d6, d_7::Day7 as y16d7, d_8::Day8 as y16d8,
        d_9::Day9 as y16d9,
    },
    y_2017::{d_1::Day1 as y17d1, d_2::Day2 as y17d2},
    y_2018::{d_1::Day1 as y18d1, d_2::Day2 as y18d2, d_3::Day3 as y18d3},
    y_2019::{d_1::Day1 as y19d1, d_2::Day2 as y19d2, d_3::Day3 as y19d3, d_4::Day4 as y19d4},
    y_2020::{d_1::Day1 as y20d1, d_2::Day2 as y20d2},
    y_2021::{d_1::Day1 as y21d1, d_2::Day2 as y21d2},
    y_2022::{
        d_1::Day1 as y22d1, d_10::Day10 as y22d10, d_11::Day11 as y22d11, d_12::Day12 as y22d12,
        d_2::Day2 as y22d2, d_3::Day3 as y22d3, d_4::Day4 as y22d4, d_5::Day5 as y22d5,
        d_6::Day6 as y22d6, d_7::Day7 as y22d7, d_8::Day8 as y22d8, d_9::Day9 as y22d9,
    },
    y_2023::{
        d_1::Day1 as y23d1, d_2::Day2 as y23d2, d_3::Day3 as y23d3, d_4::Day4 as y23d4,
        d_5::Day5 as y23d5,
    },
    y_2024::d_1::Day1 as y24d1,
};
use clap::Parser;

// #[command(version, about, long_about = None)]
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    year: Option<usize>,

    /// Number of times to greet
    #[arg(short, long)]
    day: Option<usize>,
}

fn run(q: &mut dyn Questions, question_number: QuestionNumber) {
    match question_number {
        QuestionNumber::One => q.question_one().expect("error running question 1"),
        QuestionNumber::Two => q.question_two().expect("error running question 2"),
    };

    ()
}

fn main() {
    let title = "Advent of code!";
    println!("{}", title.yellow().bold());

    let year;
    let day;

    let args = Args::parse();

    let years = vec![2015, 2016, 2017, 2018, 2019, 2020, 2021, 2022, 2023, 2024];

    if args.year.is_some() {
        year = args.year.unwrap();
    } else {
        year = years[prompt_select("Select the year", &years)];
    }

    if args.day.is_some() {
        day = args.day.unwrap();
    } else {
        day = prompt("Type in the problem number", "1")
            .parse::<usize>()
            .expect("expected the day to be an integer");
    }

    let file = &format!("inputs/{}/{day}.txt", year);

    println!("\nfetched inputs from `{}` file!\n", file.yellow());

    let mut y15: Vec<Box<dyn Questions>> = vec![
        Box::new(y15d1::new()),
        Box::new(y15d2::new()),
        Box::new(y15d3::new()),
        Box::new(y15d4::new()),
        Box::new(y15d5::new()),
        Box::new(y15d6::new()),
        Box::new(y15d7::new()),
        Box::new(y15d8::new()),
        Box::new(y15d9::new()),
        Box::new(y15d10::new(false)),
        Box::new(y15d11::new(false)),
        Box::new(y15d12::new()),
    ];

    let mut y16: Vec<Box<dyn Questions>> = vec![
        Box::new(y16d1::new()),
        Box::new(y16d2::new()),
        Box::new(y16d3::new()),
        Box::new(y16d4::new()),
        Box::new(y16d5::new()),
        Box::new(y16d6::new()),
        Box::new(y16d7::new()),
        Box::new(y16d8::new((50, 6))),
        Box::new(y16d9::new()),
    ];

    let mut y17: Vec<Box<dyn Questions>> = vec![Box::new(y17d1::new()), Box::new(y17d2::new())];

    let mut y18: Vec<Box<dyn Questions>> = vec![
        Box::new(y18d1::new()),
        Box::new(y18d2::new()),
        Box::new(y18d3::new()),
    ];

    let mut y19: Vec<Box<dyn Questions>> = vec![
        Box::new(y19d1::new()),
        Box::new(y19d2::new()),
        Box::new(y19d3::new()),
        Box::new(y19d4::new()),
    ];

    let mut y20: Vec<Box<dyn Questions>> = vec![Box::new(y20d1::new()), Box::new(y20d2::new())];

    let mut y21: Vec<Box<dyn Questions>> = vec![Box::new(y21d1::new()), Box::new(y21d2::new())];

    let mut y22: Vec<Box<dyn Questions>> = vec![
        Box::new(y22d1::new()),
        Box::new(y22d2::new()),
        Box::new(y22d3::new()),
        Box::new(y22d4::new()),
        Box::new(y22d5::new(false)),
        Box::new(y22d6::new()),
        Box::new(y22d7::new()),
        Box::new(y22d8::new()),
        Box::new(y22d9::new()),
        Box::new(y22d10::new()),
        Box::new(y22d11::new(false)),
        Box::new(y22d12::new()),
    ];

    let mut y23: Vec<Box<dyn Questions>> = vec![
        Box::new(y23d1::new()),
        Box::new(y23d2::new()),
        Box::new(y23d3::new()),
        Box::new(y23d4::new()),
        Box::new(y23d5::new()),
    ];

    let mut y24: Vec<Box<dyn Questions>> = vec![Box::new(y24d1::new())];

    let mut problem: &mut dyn Questions = y22[0].as_mut();

    match year {
        2015 => problem = y15[day - 1].as_mut(),
        2016 => problem = y16[day - 1].as_mut(),
        2017 => problem = y17[day - 1].as_mut(),
        2018 => problem = y18[day - 1].as_mut(),
        2019 => problem = y19[day - 1].as_mut(),
        2020 => problem = y20[day - 1].as_mut(),
        2021 => problem = y21[day - 1].as_mut(),
        2022 => problem = y22[day - 1].as_mut(),
        2023 => problem = y23[day - 1].as_mut(),
        2024 => problem = y24[day - 1].as_mut(),
        _ => println!("Still not implemented"),
    }

    let start = Instant::now();

    problem
        .init(file)
        .expect("error trying to initialize the problem");

    let msg = format!(
        "Initialization (consuming and parsing file - y{}|d{}) => {} µs",
        year,
        day,
        start.elapsed().as_micros()
    );

    println!("{}", msg.blue().bold());

    let start = Instant::now();

    run(problem, QuestionNumber::One);
    let msg = format!(
        "Time taken for 1st question: {} µs",
        start.elapsed().as_micros()
    );
    println!("{}", msg.blue().bold());

    let start = Instant::now();
    run(problem, QuestionNumber::Two);
    let msg = format!(
        "Time taken for 2nd question: {} µs",
        start.elapsed().as_micros()
    );
    println!("{}", msg.blue().bold());
}
