use num_bigint::BigUint;

pub fn solve() -> u64 {
    let mut result = 2;

    let (mut a, mut b) = (BigUint::new(vec![1]), BigUint::new(vec![1]));

    while b.to_string().len() < 1000 {
        (a, b) = (b.clone(), a + b);
        result += 1;
    }

    result
}
