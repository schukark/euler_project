use std::time::Instant;

mod problem_132;
mod problem_137;
mod problem_138;
mod problem_139;
mod problem_145;

pub fn solve(problem_number: u64) {
    let start = Instant::now();

    let result = match problem_number {
        132 => problem_132::solve(),
        137 => problem_137::solve(),
        138 => problem_138::solve(),
        139 => problem_139::solve(),
        145 => problem_145::solve(),
        _ => panic!("Incorrect problem number"),
    };

    println!("Problem {problem_number}: {result}");

    let execution_time = Instant::now() - start;
    println!("Time spent: {}s", execution_time.as_secs_f64());
}
