use std::fs;

pub fn read_file(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}

pub fn read_grid<T>(path: &str) -> Vec<Vec<T>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    fs::read_to_string(path)
        .unwrap()
        .split('\n')
        .map(|s| s.to_owned())
        .map(|g| {
            g.split_whitespace()
                .map(|c| c.parse::<T>().unwrap())
                .collect::<Vec<T>>()
        })
        .collect::<Vec<Vec<T>>>()
}

pub fn read_list(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|s| s.trim_end_matches('"').trim_start_matches('"').to_owned())
        .collect::<Vec<String>>()
}
