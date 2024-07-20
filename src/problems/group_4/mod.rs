use std::time::Instant;

mod problem_76;
// mod problem_77;
mod problem_78;
// mod problem_79;
// mod problem_80;
mod problem_81;
mod problem_82;
mod problem_83;
// mod problem_84;
mod problem_85;
// mod problem_86;
mod problem_87;
// mod problem_88;
// mod problem_89;
// mod problem_90;
// mod problem_91;
mod problem_92;
// mod problem_93;
// mod problem_94;
// mod problem_95;
// mod problem_96;
// mod problem_97;
// mod problem_98;
// mod problem_99;
// mod problem_100;

pub fn solve(problem_number: u64) {
    let start = Instant::now();

    let result = match problem_number {
        76 => problem_76::solve(100),
        78 => problem_78::solve(),
        81 => problem_81::solve(),
        82 => problem_82::solve(),
        83 => problem_83::solve(),
        85 => problem_85::solve(),
        // 86 => problem_86::solve(),
        87 => problem_87::solve(),
        92 => problem_92::solve(),

        _ => panic!("Invalid problem number"),
    };

    println!("Problem {problem_number}: {result}");

    let execution_time = Instant::now() - start;
    println!("Time spent: {}s", execution_time.as_secs_f64());
}
