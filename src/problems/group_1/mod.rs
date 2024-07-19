use std::time::Instant;

mod problem_1;
mod problem_10;
mod problem_11;
mod problem_12;
mod problem_13;
mod problem_14;
mod problem_15;
mod problem_16;
mod problem_17;
mod problem_18;
mod problem_19;
mod problem_2;
mod problem_20;
mod problem_21;
mod problem_22;
mod problem_23;
mod problem_24;
mod problem_25;
mod problem_3;
mod problem_4;
mod problem_5;
mod problem_6;
mod problem_7;
mod problem_8;
mod problem_9;

pub fn solve(problem_number: u64) {
    let start = Instant::now();

    let result = match problem_number {
        1 => problem_1::solve(1000),
        2 => problem_2::solve(),
        3 => problem_3::solve(600851475143),
        4 => problem_4::solve(3),
        5 => problem_5::solve(20),
        6 => problem_6::solve(100),
        7 => problem_7::solve(10001),
        8 => problem_8::solve(),
        9 => problem_9::solve(1000),
        10 => problem_10::solve(2_000_000),
        11 => problem_11::solve(),
        12 => problem_12::solve(500),
        13 => problem_13::solve(),
        14 => problem_14::solve(1_000_000),
        15 => problem_15::solve(20, 20),
        16 => problem_16::solve(1000),
        17 => problem_17::solve(1000),
        18 => problem_18::solve(),
        19 => problem_19::solve(),
        20 => problem_20::solve(),
        21 => problem_21::solve(10000),
        22 => problem_22::solve(),
        23 => problem_23::solve(),
        24 => problem_24::solve(),
        25 => problem_25::solve(),
    _ => panic!("Invalid problem number"),
    };

    println!("Problem {problem_number}: {result}");

    let execution_time = Instant::now() - start;
    println!("Time spent: {}s", execution_time.as_secs_f64());
}
