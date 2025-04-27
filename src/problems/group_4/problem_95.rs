use std::collections::HashSet;

const LIMIT: i64 = 1_000_000;

pub fn solve() -> i128 {
    let mut longest = vec![];

    for i in 1..=LIMIT {
        let mut visited = HashSet::new();

        let mut cur_chain = vec![i];
        visited.insert(i);

        let mut vertex = calc_sum_divisors(i);

        while vertex <= LIMIT && !visited.contains(&vertex) {
            cur_chain.push(vertex);
            visited.insert(vertex);
            vertex = calc_sum_divisors(vertex);
        }

        if vertex != i {
            continue;
        }

        if longest.len() < cur_chain.len() {
            longest = cur_chain;
        }
    }

    dbg!(&longest);

    longest.into_iter().min().unwrap() as i128
}

fn calc_sum_divisors(cur: i64) -> i64 {
    (2..=cur.isqrt())
        .filter(|x| cur % x == 0)
        .map(|x| x + cur / x - x * (x * x == cur) as i64)
        .sum::<i64>()
        + 1
}

#[cfg(test)]
mod tests {
    use crate::problems::group_4::problem_95::calc_sum_divisors;

    #[test]
    fn test_example_1() {
        assert_eq!(220, calc_sum_divisors(284));
    }

    #[test]
    fn test_example_2() {
        assert_eq!(284, calc_sum_divisors(220));
    }

    #[test]
    fn test_example_3() {
        assert_eq!(14288, calc_sum_divisors(12496));
    }
}
