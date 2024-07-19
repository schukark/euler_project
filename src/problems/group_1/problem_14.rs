use std::collections::HashMap;

fn collatz(number: u64, lengths: &mut HashMap<u64, u64>) -> u64 {
    let mut cur_num = number;
    let mut count = 0;

    while cur_num != 1 {
        if lengths.get(&cur_num).is_some() {
            let map_entry = *lengths.get_mut(&cur_num).unwrap();
            lengths.insert(number, map_entry + count);
            return map_entry + count;
        }

        if cur_num % 2 == 0 {
            cur_num /= 2;
        } else {
            cur_num = 3 * cur_num + 1;
        }

        count += 1;
    }

    count
}

pub fn solve(limit: u64) -> u64 {
    let mut max_num = 0;
    let mut max_count = 0;

    let mut map: HashMap<u64, u64> = HashMap::new();

    for i in 1..=limit {
        let tmp = collatz(i, &mut map);

        if tmp > max_count {
            max_count = tmp;
            max_num = i;
        }
    }

    max_num
}
