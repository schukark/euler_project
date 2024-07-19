use num_bigint::BigUint;

pub fn solve(limit: u64) -> i128 {
    let mut num_start = (BigUint::new(vec![3]), BigUint::new(vec![7]));
    let mut denom_start = (BigUint::new(vec![2]), BigUint::new(vec![5]));

    let mut count = 0;

    for _i in 3..=limit {
        let (a, b) = num_start;
        num_start = (b.clone(), a + BigUint::new(vec![2]) * b);

        let (a, b) = denom_start;
        denom_start = (b.clone(), a + BigUint::new(vec![2]) * b);

        if denom_start.1.to_string().len() < num_start.1.to_string().len() {
            count += 1;
        }
    }

    count
}

#[test]
pub fn test_expansion() {
    assert_eq!(solve(8), 1);
}
