use std::env;

pub mod problems;
pub mod utils;

mod prob_51_75;

fn main() {
    let problem_number = env::args().collect::<Vec<_>>()[1].to_owned();

    if problem_number.parse::<u64>().is_err() {
        panic!("Problem number was incorrect");
    }

    let problem_number = problem_number.parse::<u64>().unwrap();

    match problem_number {
        1..=25 => problems::group_1::solve(problem_number),
        26..=50 => problems::group_2::solve(problem_number),
        _ => println!("Incorrect config"),
    };
}
