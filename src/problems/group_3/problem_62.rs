use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn digits(number: i128) -> [u32; 10] {
    let mut digits = [0; 10];

    number
        .to_string()
        .chars()
        .for_each(|digit| digits[digit.to_digit(10).unwrap() as usize] += 1);

    digits
}

pub fn solve(limit: u32) -> i128 {
    let mut lookup: HashMap<[u32; 10], Vec<u32>> = HashMap::new();

    let mut cur_number = 1;

    loop {
        let digits_cur = digits(cur_number * cur_number * cur_number);
        if let Entry::Vacant(e) = lookup.entry(digits_cur) {
            e.insert(vec![cur_number as u32]);
        } else {
            lookup.get_mut(&digits_cur).unwrap().push(cur_number as u32);
        }

        if lookup
            .get(&digits_cur)
            .is_some_and(|f| f.len() == limit as usize)
        {
            return (*lookup.get(&digits_cur).unwrap().first().unwrap() as i128).pow(3);
        }

        cur_number += 1;
    }
}
