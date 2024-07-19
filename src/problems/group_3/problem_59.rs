use std::fs;

fn read_words() -> Vec<String> {
    let mut collect = fs::read_to_string("src/txts/common_english.txt")
        .unwrap()
        .split('\n')
        .map(|a| a.trim().to_owned())
        .collect::<Vec<_>>();
    collect.sort();
    collect
}

fn read_encrypted() -> Vec<u8> {
    fs::read_to_string("src/txts/prob_59.txt")
        .unwrap()
        .split(',')
        .map(|a| a.parse::<u8>().unwrap())
        .collect::<Vec<_>>()
}

fn decrypt(data: &[u8], cypher: &[char]) -> Vec<u8> {
    assert!(data.len() % cypher.len() == 0);

    let mut new_data = data.to_vec();

    for i in 0..data.len() {
        new_data[i] ^= cypher[i % cypher.len()] as u8;
    }

    new_data
}

fn check_common_words(data: &[u8], words: &[String]) -> bool {
    data.iter()
        .map(|a| *a as char)
        .map(|a| a.to_ascii_lowercase().to_string())
        .collect::<Vec<_>>()
        .join("")
        .split_whitespace()
        .filter(|word| words.binary_search(&word.to_owned().to_lowercase()).is_ok())
        .count() as f32
        / data
            .iter()
            .map(|a| *a as char)
            .map(|a| a.to_ascii_lowercase().to_string())
            .collect::<Vec<_>>()
            .join("")
            .split_whitespace()
            .count() as f32
        > 0.5
}

pub fn solve() -> i128 {
    let words = &read_words();
    let data = read_encrypted();

    for letter1 in 'a'..='z' {
        for letter2 in 'a'..='z' {
            for letter3 in 'a'..='z' {
                let cypher = [letter1, letter2, letter3];
                if check_common_words(&decrypt(&data, &cypher), words) {
                    return decrypt(&data, &cypher)
                        .iter()
                        .map(|a| *a as i128)
                        .sum::<i128>();
                }
            }
        }
    }

    0
}

#[test]
pub fn test_decryption() {
    let data = vec![65, 42, 107];
    let cypher = ['*', 'A', 'k'];
    assert_eq!(decrypt(&data, &cypher), vec![107, 107, 0]);
}

#[test]
pub fn test_common_words() {
    let data = "Hello rust language project maths"
        .chars()
        .map(|a| a as u8)
        .collect::<Vec<_>>();
    let words = read_words();

    assert!(check_common_words(&data, &words));
}
