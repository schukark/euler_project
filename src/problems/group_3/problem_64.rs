use std::fmt::Display;

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct RootFraction {
    square_root: i128,
    addend: i128,
    coef: i128,
    denominator: i128,
}

impl Display for RootFraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}sqrt({})+{})/{}",
            self.coef, self.square_root, self.addend, self.denominator
        )
    }
}

impl RootFraction {
    pub fn new(square_root: i128, addend: i128, coef: i128, denominator: i128) -> RootFraction {
        let gcd = gcd::binary_u128(
            gcd::binary_u128(i128::abs(addend) as u128, i128::abs(coef) as u128),
            i128::abs(denominator) as u128,
        ) as i128;

        RootFraction {
            square_root,
            addend: addend / gcd,
            coef: coef / gcd,
            denominator: denominator / gcd,
        }
    }

    pub fn find_floor(&self) -> i128 {
        f64::floor(
            (f64::sqrt(self.square_root as f64) + self.addend as f64) / self.denominator as f64,
        ) as i128
    }

    pub fn subtract_biggest(&self) -> RootFraction {
        let biggest = self.find_floor();

        RootFraction {
            square_root: self.square_root,
            addend: self.addend - biggest * self.denominator,
            coef: self.coef,
            denominator: self.denominator,
        }
    }

    pub fn invert(&self) -> RootFraction {
        RootFraction::new(
            self.square_root,
            -self.addend * self.denominator,
            self.coef * self.denominator,
            self.coef * self.coef * self.square_root - self.addend * self.addend,
        )
    }
}

fn find_cycle(square_root: i128) -> usize {
    if f64::floor(f64::sqrt(square_root as f64)).powf(2.0) as i128 == square_root {
        return 0;
    }

    let mut result = Vec::new();

    let mut current = RootFraction::new(square_root, 0, 1, 1)
        .subtract_biggest()
        .invert();
    let start = current;

    result.push(current.find_floor());
    current = current.subtract_biggest().invert();

    while current != start {
        result.push(current.find_floor());

        current = current.subtract_biggest().invert();
    }

    result.len()
}

pub fn solve(limit: i128) -> i128 {
    let mut result = 0;

    for i in 2..=limit {
        if find_cycle(i) % 2 == 1 {
            result += 1;
        }
    }

    result
}

#[test]
pub fn test_find_cycle() {
    assert_eq!(find_cycle(2), 1);
    assert_eq!(find_cycle(3), 2);
    assert_eq!(find_cycle(7), 4);
    assert_eq!(find_cycle(13), 5);
}
