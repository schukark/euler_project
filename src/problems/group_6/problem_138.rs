use std::collections::BTreeSet;

pub fn solve() -> i128 {
    const LIMIT: i128 = 12;
    let mut values: BTreeSet<i128> = BTreeSet::new();

    let (mut a, mut b): (i128, i128) = (9, 4);

    //case 1: x^2 - 5y^2 = 1
    for _i in 0..LIMIT {
        values.insert((a + 2 * b).pow(2) + b.pow(2));

        (a, b) = (9 * a + 20 * b, 4 * a + 9 * b);
        assert_eq!(a.pow(2) - 5 * b.pow(2), 1);
    }

    //case 2: x^2 - 5y^2 = -1
    let (mut a, mut b): (i128, i128) = (2, 1);
    for _i in 0..LIMIT {
        values.insert((a + 2 * b).pow(2) + b.pow(2));

        (a, b) = (9 * a + 20 * b, 4 * a + 9 * b);
        assert_eq!(a.pow(2) - 5 * b.pow(2), -1);
    }

    while values.len() > LIMIT as usize {
        values.pop_last();
    }

    println!("{:#?}, len is {}", values, values.len());

    values.iter().sum::<i128>()
}
