fn count_letters(number: u64) -> usize {
    assert!(number <= 1000);

    if number == 1000 {
        return "onethousand".len();
    }

    let letters = [0, 3, 3, 5, 4, 4, 3, 5, 5, 4];
    let letters_tens = [0, 3, 6, 6, 5, 5, 5, 7, 6, 6];

    let mut result = 0;

    if number >= 100 {
        result += letters[(number / 100) as usize];
        result += "hundred".len();

        if number % 100 != 0 {
            result += "and".len();
        } else {
            return result;
        }
    }

    let number = number % 100;

    if !(11..20).contains(&number) {
        result += letters_tens[(number / 10) as usize] + letters[(number % 10) as usize];
    } else {
        result += match number {
            0..=10 => 0,
            11 => "eleven".len(),
            12 => "twelve".len(),
            13 => "thirteen".len(),
            14 => "fourteen".len(),
            15 => "fifteen".len(),
            16 => "sixteen".len(),
            17 => "seventeen".len(),
            18 => "eighteen".len(),
            19 => "nineteen".len(),
            20..=u64::MAX => 0,
        }
    }

    result
}

pub fn solve(limit: u64) -> u64 {
    (1..=limit).map(count_letters).sum::<usize>() as u64
}

#[test]
pub fn test_letter_count() {
    assert_eq!(count_letters(342), 23);
    assert_eq!(count_letters(115), 20);
    assert_eq!(count_letters(105), 17);
    assert_eq!(count_letters(555), "fivehundredandfiftyfive".len());
    assert_eq!(count_letters(25), "twentyfive".len());
    assert_eq!(count_letters(5), "five".len());
    assert_eq!(count_letters(110), "onehundredandten".len());
    assert_eq!(count_letters(900), "ninehundred".len());
}
