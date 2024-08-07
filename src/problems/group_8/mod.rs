use std::time::Instant;

mod problem_187;

pub fn solve(problem_number: u64) {
    let start = Instant::now();

    let result = match problem_number {
        187 => problem_187::solve(),
        _ => panic!("Incorrect problem number"),
    };

    println!("Problem {problem_number}: {result}");

    let execution_time = Instant::now() - start;
    println!("Time spent: {}s", execution_time.as_secs_f64());
}
