pub fn solve(limit: i128) -> i128 {
    let limit = (limit + 1) / 2;

    // TR diagonal: (2n-1)^2
    // BR diagonal: 4n^2-10n+7
    // BL diagonal: 4(n-1)^2+1
    // TL diagonal: 4n^2 - 6n + 3
    // TOTAL per n: 16n^2-28n+16

    // sum_{i=1}^{(limit - 1) / 2} 16n^2-20n+12=16/6 N (N+1)(2N+1) - 28/2 N(N+1) + 16 N=
    // 8/3 (2N^3+3N^2+N)-14N^2-14N+16N = 16/3N^3 - 6N^2 + 14/3N

    // Amount = 16/3N^3 - 6N^2 + 14/3N - 3

    (16 * limit * limit * limit - 18 * limit * limit + 14 * limit - 9) / 3
}
