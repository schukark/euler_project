#![feature(isqrt, iter_array_chunks)]

use std::env;

pub mod problems;
pub mod utils;

fn main() {
    let problem_number = env::args().collect::<Vec<_>>()[1].to_owned();

    if problem_number.parse::<u64>().is_err() {
        panic!("Problem number was incorrect");
    }

    let problem_number = problem_number.parse::<u64>().unwrap();

    match problem_number {
        1..=25 => problems::group_1::solve(problem_number),
        26..=50 => problems::group_2::solve(problem_number),
        51..=75 => problems::group_3::solve(problem_number),
        76..=100 => problems::group_4::solve(problem_number),
        101..=125 => problems::group_5::solve(problem_number),
        126..=150 => problems::group_6::solve(problem_number),
        151..=175 => problems::group_7::solve(problem_number),
        176..=200 => problems::group_8::solve(problem_number),
        201..=225 => problems::group_9::solve(problem_number),
        226..=250 => problems::group_9::solve(problem_number),

        351..=375 => problems::group_15::solve(problem_number),

        _ => println!("Incorrect config"),
    };
}
