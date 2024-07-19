use std::time::Instant;

mod problem_26;
mod problem_27;
mod problem_28;
mod problem_29;
mod problem_30;
mod problem_31;
mod problem_32;
mod problem_33;
mod problem_34;
mod problem_35;
mod problem_36;
mod problem_37;
mod problem_38;
mod problem_39;
mod problem_40;
mod problem_41;
mod problem_42;
mod problem_43;
mod problem_44;
mod problem_45;
mod problem_46;
mod problem_47;
mod problem_48;
mod problem_49;
mod problem_50;

pub fn solve(problem_number: u64) {
    let start = Instant::now();

    let result = match problem_number {
        26 => problem_26::solve(1000),
        27 => problem_27::solve(),
        28 => problem_28::solve(1001),
        29 => problem_29::solve(),
        30 => problem_30::solve(),
        31 => problem_31::solve(200),
        32 => problem_32::solve(),
        33 => problem_33::solve(),
        34 => problem_34::solve(),
        35 => problem_35::solve(1_000_000),
        36 => problem_36::solve(),
        37 => problem_37::solve(),
        38 => problem_38::solve(),
        39 => problem_39::solve(),
        40 => problem_40::solve(),
        41 => problem_41::solve(),
        42 => problem_42::solve(),
        43 => problem_43::solve(),
        44 => problem_44::solve(),
        45 => problem_45::solve(),
        46 => problem_46::solve(),
        47 => problem_47::solve(4),
        48 => problem_48::solve(1_000),
        49 => problem_49::solve(),
        50 => problem_50::solve(1_000_000),
        _ => panic!("Invalid problem number"),
    };

    println!("Problem {problem_number}: {result}");

    let execution_time = Instant::now() - start;
    println!("Time spent: {}s", execution_time.as_secs_f64());
}
