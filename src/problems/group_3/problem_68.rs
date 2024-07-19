use std::collections::HashSet;

pub fn solve() -> i128 {
    let mut result = i128::MIN;

    for mask in 0..=10_i32.pow(6) {
        let mut digits = mask
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i128 + 1)
            .collect::<Vec<i128>>();

        if digits
            .clone()
            .iter()
            .copied()
            .collect::<HashSet<i128>>()
            .len()
            != 6
        {
            continue;
        }

        let cur_sum = digits[0] + digits[1] + digits[2];
        let mut should_break = false;

        for i in 2..6 {
            if cur_sum - digits[i] - digits[i + 1] <= 0 || cur_sum - digits[i] - digits[i + 1] > 10
            {
                should_break = true;
                break;
            }

            digits.push(cur_sum - digits[i] - digits[i % 5 + 1]);
        }

        if should_break {
            continue;
        }

        if digits
            .clone()
            .iter()
            .copied()
            .collect::<HashSet<i128>>()
            .len()
            != 10
        {
            continue;
        }

        let triples: [(i128, i128, i128); 5] = [
            (digits[0], digits[1], digits[2]),
            (digits[6], digits[2], digits[3]),
            (digits[7], digits[3], digits[4]),
            (digits[8], digits[4], digits[5]),
            (digits[9], digits[5], digits[1]),
        ];

        if triples
            .iter()
            .map(|triple| triple.0.to_string() + &triple.1.to_string() + &triple.2.to_string())
            .collect::<Vec<String>>()
            .join("")
            .len()
            != 16
        {
            continue;
        }

        let triples = min_rotation(&triples);
        let parse = triples
            .iter()
            .map(|triple| triple.0.to_string() + &triple.1.to_string() + &triple.2.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<i128>()
            .unwrap();

        result = i128::max(parse, result);
    }

    result
}

fn min_rotation(triples: &[(i128, i128, i128); 5]) -> Vec<(i128, i128, i128)> {
    let min_index = triples
        .iter()
        .enumerate()
        .min_by_key(|(_, b)| b.0)
        .unwrap()
        .0;

    [&triples[min_index..], &triples[..min_index]].concat()
}
