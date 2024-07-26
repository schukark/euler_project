use crate::utils::primes::prime_number_sieve;

fn check_divisible(repetitions: i32, factor: i32) -> bool {
    let mut remainders: Vec<i32> = vec![1];

    for _i in 1..repetitions {
        remainders.push((remainders.last().unwrap() * 10 + 1) % factor);

        if *remainders.last().unwrap() == 1 {
            remainders.pop();
            break;
        }
    }

    remainders[(repetitions as usize - 1) % remainders.len()] == 0
}

#[test]
fn check_example() {
    assert!(check_divisible(10, 11));
    assert!(check_divisible(10, 41));
    assert!(check_divisible(10, 271));
    assert!(check_divisible(10, 9091));
}

pub fn solve() -> i128 {
    const REPETITIONS: i32 = 10_i32.pow(9);
    const PRIME_COUNT: i32 = 40;

    let primes = prime_number_sieve(10_usize.pow(8));

    let mut found = 0;
    let mut sum = 0;

    for prime in primes {
        if found == PRIME_COUNT {
            return sum;
        }

        if check_divisible(REPETITIONS, prime) {
            found += 1;
            sum += prime as i128;
        }
    }

    todo!()
}
