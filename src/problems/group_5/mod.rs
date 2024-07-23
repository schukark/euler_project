use std::time::Instant;

mod problem_101;
mod problem_104;

pub fn solve(problem_number: u64) {
    let start = Instant::now();

    let result = match problem_number {
        101 => problem_101::solve(),
        104 => problem_104::solve(),
        _ => panic!("Incorrect problem number"),
    };

    println!("Problem {problem_number}: {result}");

    let execution_time = Instant::now() - start;
    println!("Time spent: {}s", execution_time.as_secs_f64());
}
