use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::utils::primes::get_prime_exponents;

fn count_solutions(number: i64) -> i64 {
    ((get_prime_exponents(number)
        .iter()
        .map(|(_prime, exponent)| { *exponent } * 2 + 1)
        .product::<i64>()
        + 1)
        / 2) as i64
}

const LIMIT: i64 = 1_000;
const THREAD_COUNT: i64 = 12;

fn do_task(index: i64, thread_count: i64, answer: Arc<Mutex<Option<i64>>>) {
    let mut current = index;

    loop {
        let count = count_solutions(current);

        if count <= LIMIT {
            current += thread_count;
            continue;
        }

        let mut tmp = answer.lock().unwrap();

        if tmp.is_none() {
            *tmp = Some(current);
        }

        if tmp.unwrap() > current {
            *tmp = Some(current);
        }

        break;
    }
}

pub fn solve() -> i128 {
    let answer = Arc::new(Mutex::new(Option::None));

    let mut handles = Vec::new();

    for i in 0..THREAD_COUNT {
        let copy = Arc::clone(&answer);
        let handle = thread::spawn(move || {
            do_task(i, THREAD_COUNT, copy);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread didn't join successfully");
    }

    let x = answer.lock().unwrap().unwrap() as i128;
    x
}
