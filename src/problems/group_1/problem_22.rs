use crate::utils::file_ops::read_list;

fn sort_names() -> Vec<String> {
    let mut collect = read_list("src/txts/prob_22.txt");
    collect.sort();
    collect
}

fn calculate_score(name: &str) -> u64 {
    name.chars().map(|a| (a as u8 - b'A') as u64 + 1).sum()
}

pub fn solve() -> u64 {
    sort_names()
        .iter()
        .enumerate()
        .map(|(a, b)| calculate_score(b) * ((a + 1) as u64))
        .sum()
}

#[test]
pub fn test_score() {
    assert_eq!(calculate_score("COLIN"), 53);
}

#[test]
pub fn test_sort() {
    assert_eq!(sort_names()[937], "COLIN");
}
