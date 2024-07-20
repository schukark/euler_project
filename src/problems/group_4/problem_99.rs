use std::fs::read_to_string;

fn read_pairs() -> Vec<Vec<f64>> {
    read_to_string("src/txts/prob_99.txt")
        .unwrap()
        .split('\n')
        .map(|row| {
            row.split(',')
                .map(|number| number.parse::<f64>().unwrap())
                .take(2)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn solve() -> i128 {
    let data = read_pairs();
    data.iter()
        .enumerate()
        .max_by(|a, b| (a.1[1] * a.1[0].log2()).total_cmp(&(b.1[1] * b.1[0].log2())))
        .unwrap()
        .0 as i128
        + 1
}
