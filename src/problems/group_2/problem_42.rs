use crate::utils::file_ops::read_list;

fn is_triangular(number: i128) -> bool {
    // n(n + 1) / 2 = k
    // n ^ 2 + n - 2k = 0
    // n = (-1 + sqrt(1 + 8k)) / 2

    let n = (f64::sqrt(1.0_f64 + 8.0_f64 * number as f64) - 1.0_f64) / 2.0_f64;
    f64::abs(n - n.floor()) < 1e-5
}

fn calculate_score(word: &str) -> i128 {
    word.chars().map(|a| (a as u8 - b'A' + 1) as i128).sum()
}

pub fn solve() -> i128 {
    let mut count = 0;

    let words = read_list("src/txts/prob_42.txt");

    for word in &words {
        let score = calculate_score(word);

        if is_triangular(score) {
            count += 1;
        }
    }

    count
}

#[test]
pub fn test_everything() {
    assert_eq!(calculate_score("SKY"), 55);
    assert!(is_triangular(calculate_score("SKY")));
}
