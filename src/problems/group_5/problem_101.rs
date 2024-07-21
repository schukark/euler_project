use crate::utils::{fraction::Fraction, polynomial::Polynomial};

fn find_bop(example: &Polynomial, degree: i32) -> Fraction {
    let data = (1..=degree)
        .map(|x| example.eval(x.into()))
        .collect::<Vec<_>>();

    let fit = Polynomial::fit(&(1..=degree).map(|x| x.into()).collect::<Vec<_>>(), &data);

    for i in 1.. {
        if fit.eval(i.into()) != example.eval(i.into()) {
            return fit.eval(i.into());
        }
    }

    0.into()
}

#[test]
fn test_bop() {
    let coefs = &[0.into(), 0.into(), 0.into(), 1.into()];
    let example = Polynomial::new(coefs);

    assert_eq!(find_bop(&example, 3), 58.into());
}

pub fn solve() -> i128 {
    let coefs = &[
        1.into(),
        (-1).into(),
        1.into(),
        (-1).into(),
        1.into(),
        (-1).into(),
        1.into(),
        (-1).into(),
        1.into(),
        (-1).into(),
        1.into(),
    ]; // 1 - n + n^2 - ... + n^10
    let example = Polynomial::new(coefs);

    let mut answer: Fraction = 0.into();

    for i in 1..coefs.len() {
        answer += find_bop(&example, i as i32);
    }

    answer.numerator
}
