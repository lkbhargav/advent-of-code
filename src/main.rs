use advent_of_code::{
    prelude::*,
    y_2015::{d_1::*, d_21::Day21},
    y_2022::d_01::Day01 as y22d1,
};
use colored::*;

fn run<T>(q: &mut T, question_number: QuestionNumber)
where
    T: Questions,
{
    match question_number {
        QuestionNumber::One => q.question_one().expect("error running question 1"),
        QuestionNumber::Two => q.question_two().expect("error running question 2"),
    }
}

fn main() {
    let years = vec![2015, 2016];
    prompt_select("Select the year", &years);

    let mut d1 = y22d1::new();
    run(&mut d1, QuestionNumber::One);
    run(&mut d1, QuestionNumber::Two);
}
