use std::time::Instant;

use advent_of_code::{
    prelude::*,
    y_2015::{
        d_1::Day1 as y15d1, d_10::Day10 as y15d10, d_11::Day11 as y15d11, d_12::Day12 as y15d12,
        d_2::Day2 as y15d2, d_3::Day3 as y15d3, d_4::Day4 as y15d4, d_5::Day5 as y15d5,
        d_6::Day6 as y15d6, d_7::Day7 as y15d7, d_8::Day8 as y15d8, d_9::Day9 as y15d9,
    },
    y_2016::d_1::Day1 as y16d1,
    y_2017::{d_1::Day1 as y17d1, d_2::Day2 as y17d2},
    y_2018::{d_1::Day1 as y18d1, d_2::Day2 as y18d2},
    y_2022::{
        d_1::Day1 as y22d1, d_10::Day10 as y22d10, d_11::Day11 as y22d11, d_2::Day2 as y22d2,
        d_3::Day3 as y22d3, d_4::Day4 as y22d4, d_5::Day5 as y22d5, d_6::Day6 as y22d6,
        d_7::Day7 as y22d7, d_8::Day8 as y22d8, d_9::Day9 as y22d9,
    },
    y_2023::{d_1::Day1 as y23d1, d_2::Day2 as y23d2},
};

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

    let years = vec![2015, 2016, 2017, 2018, 2022, 2023];
    let year = prompt_select("Select the year", &years);
    let day = prompt("Type in the problem number", "1")
        .parse::<usize>()
        .expect("expected the day to be an integer");

    let file = &format!("inputs/{}/{day}.txt", years[year]);

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

    let mut y16: Vec<Box<dyn Questions>> = vec![Box::new(y16d1::new())];

    let mut y17: Vec<Box<dyn Questions>> = vec![Box::new(y17d1::new()), Box::new(y17d2::new())];

    let mut y18: Vec<Box<dyn Questions>> = vec![Box::new(y18d1::new()), Box::new(y18d2::new())];

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
    ];

    let mut y23: Vec<Box<dyn Questions>> = vec![Box::new(y23d1::new()), Box::new(y23d2::new())];

    let mut problem: &mut dyn Questions = y22[0].as_mut();

    match years[year] {
        2015 => problem = y15[day - 1].as_mut(),
        2016 => problem = y16[day - 1].as_mut(),
        2017 => problem = y17[day - 1].as_mut(),
        2018 => problem = y18[day - 1].as_mut(),
        2022 => problem = y22[day - 1].as_mut(),
        2023 => problem = y23[day - 1].as_mut(),
        _ => println!("Still not implemented"),
    }

    let start = Instant::now();

    problem
        .init(file)
        .expect("error trying to initialize the problem");

    let msg = format!(
        "Initialization (consuming and parsing file - y{}|d{}) => {} µs",
        years[year],
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
