use std::time::Instant;

use crate::utils::fraction::Fraction;

mod problem_51;
mod problem_52;
mod problem_53;
mod problem_54;
mod problem_55;
mod problem_56;
mod problem_57;
mod problem_58;
mod problem_59;
mod problem_60;
mod problem_61;
mod problem_62;
mod problem_63;
mod problem_64;
mod problem_65;
mod problem_66;
mod problem_67;
mod problem_68;
mod problem_69;
mod problem_70;
mod problem_71;
mod problem_72;
mod problem_73;
mod problem_74;
mod problem_75;

pub fn solve(problem_number: u64) {
    let start = Instant::now();

    let result = match problem_number {
        51 => problem_51::solve(8),
        52 => problem_52::solve(),
        53 => problem_53::solve(),
        54 => problem_54::solve(),
        55 => problem_55::solve(),
        56 => problem_56::solve(),
        57 => problem_57::solve(1000),
        58 => problem_58::solve(0.1),
        59 => problem_59::solve(),
        // 60 => problem_60::solve(5),
        61 => problem_61::solve(),
        62 => problem_62::solve(5),
        63 => problem_63::solve(),
        64 => problem_64::solve(10_000),
        65 => problem_65::solve(100),
        // 66 => problem_66::solve(),
        67 => problem_67::solve(),
        68 => problem_68::solve(),
        69 => problem_69::solve(1_000_000),
        70 => problem_70::solve(10_000_000 - 1),
        71 => problem_71::solve(Fraction::new(3, 7), 1_000_000),
        72 => problem_72::solve(1_000_000),
        73 => problem_73::solve(Fraction::new(1, 3), Fraction::new(1, 2), 12_000),
        74 => problem_74::solve(60, 1_000_000),
        75 => problem_75::solve(1_500_000),
        _ => panic!("Invalid problem number"),
    };

    println!("Problem {problem_number}: {result}");

    let execution_time = Instant::now() - start;
    println!("Time spent: {}s", execution_time.as_secs_f64());
}
