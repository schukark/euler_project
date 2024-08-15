fn is_terminating(n: i64) -> bool {
    let exact_parts = (n as f64) / f64::exp(1.0);

    let closest = (exact_parts.floor(), exact_parts.ceil());

    let best = if (n as f64 / closest.0).ln() * closest.0 > (n as f64 / closest.1).ln() * closest.1
    {
        closest.0 as i64
    } else {
        closest.1 as i64
    };

    let mut best = best / gcd::binary_u64(n as u64, best as u64) as i64;

    while best % 2 == 0 {
        best /= 2;
    }

    while best % 5 == 0 {
        best /= 5;
    }

    best == 1
}

pub fn solve() -> i128 {
    (5..=10_000)
        .map(|value| {
            if !is_terminating(value) {
                value as i128
            } else {
                -value as i128
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::problems::group_8::problem_183::is_terminating;

    #[test]
    fn test_terminating() {
        assert!(is_terminating(11));
    }

    #[test]
    fn test_non_terminating() {
        assert!(!is_terminating(8));
    }
}
