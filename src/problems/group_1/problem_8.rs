use crate::utils::file_ops::read_file;

pub fn solve() -> u64 {
    let number = read_file("src/txts/prob_8.txt").join("");

    let prod_len = 13;
    let mut max_prod = u64::MIN;

    for i in 0..number.len() - prod_len {
        let mut product: u64 = 1;

        for j in i..i + prod_len {
            product *= number.chars().nth(j).unwrap().to_digit(10).unwrap() as u64;
        }

        if product > max_prod {
            max_prod = product;
        }
    }

    max_prod
}
