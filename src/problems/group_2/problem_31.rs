pub fn solve(amount: u64) -> i128 {
    let mut count = 0;

    for c200 in 0..=amount / 200 {
        for c100 in 0..=(amount - 200 * c200) / 100 {
            for c50 in 0..=(amount - 200 * c200 - 100 * c100) / 50 {
                for c20 in 0..=(amount - 200 * c200 - 100 * c100 - 50 * c50) / 20 {
                    for c10 in 0..=(amount - 200 * c200 - 100 * c100 - 50 * c50 - 20 * c20) / 10 {
                        for c5 in
                            0..=(amount - 200 * c200 - 100 * c100 - 50 * c50 - 20 * c20 - 10 * c10)
                                / 5
                        {
                            let cur_amount = amount
                                - 200 * c200
                                - 100 * c100
                                - 50 * c50
                                - 20 * c20
                                - 10 * c10
                                - 5 * c5;

                            count += cur_amount as i128 / 2 + 1;
                        }
                    }
                }
            }
        }
    }

    count
}
