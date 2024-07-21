use crate::utils::fraction::Fraction;

pub fn solve(closest: Fraction, limit: i128) -> i128 {
    let mut best = Fraction::new(0, 1);

    for i in 2..limit {
        let guess = Fraction::new(closest.numerator * i / closest.denominator, i);

        if guess == closest {
            continue;
        }

        if guess > best {
            best = guess;
        }
    }

    best.numerator as i128
}
