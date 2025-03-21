use crate::utils::file_ops::read_file;

pub fn solve() -> i128 {
    let constraints = read_file("src/txts/prob_79.txt")
        .iter()
        .map(|line| line.as_bytes())
        .map(|line| [line[0] as char, line[1] as char, line[2] as char])
        .collect::<Vec<_>>();

    for i in 1.. {
        if check_constraints(&i.to_string(), &constraints) {
            return i;
        }
    }

    0
}

fn check_constraints(password: &str, constraints: &[[char; 3]]) -> bool {
    for constraint in constraints {
        let first = password.find(constraint[0]);
        if first.is_none() {
            return false;
        }
        let first = first.unwrap();

        let second = password[first + 1..].find(constraint[1]);
        if second.is_none() {
            return false;
        }
        let second = second.unwrap() + first + 1;

        let third = password[second + 1..].find(constraint[2]);
        if third.is_none() {
            return false;
        }
    }

    true
}
