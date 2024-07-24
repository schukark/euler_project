use std::{
    sync::{Arc, Mutex},
    thread,
};

fn reverse(number: i32) -> i32 {
    number
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
}

fn is_reversible(n: i32) -> bool {
    if n % 10 == 0 {
        return false;
    }

    (n + reverse(n))
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .all(|f| f % 2 == 1)
}

#[test]
fn test_is_reversible() {
    [36, 63, 409, 904]
        .iter()
        .for_each(|&x| assert!(is_reversible(x)));
}

const THREAD_COUNT: i32 = 12;

fn do_task(index: i32, limit: i32, answer: Arc<Mutex<i32>>) {
    let mut i = index;
    let mut local_count = 0;

    while i < limit {
        if is_reversible(i) {
            local_count += 1;
        }

        i += THREAD_COUNT;
    }

    *answer.lock().unwrap() += local_count;
}

fn _solve(limit: i32) -> i128 {
    let answer = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for i in 0..THREAD_COUNT {
        let copy = Arc::clone(&answer);
        let handle = thread::spawn(move || {
            do_task(i, limit, copy);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle
            .join()
            .expect("Join shouldn't fail in these circumstances");
    }

    let x = *answer.lock().unwrap() as i128;
    x
}

#[test]
fn test_example() {
    assert_eq!(_solve(1_000), 120);
}

pub fn solve() -> i128 {
    _solve(10_i32.pow(9))
}
