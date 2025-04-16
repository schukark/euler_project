const fn modular_exponent(mut n: i128, mut x: i128, p: i128) -> i128 {
    let mut ans = 1;
    if x <= 0 {
        return 1;
    }
    loop {
        if x == 1 {
            return (ans * n) % p;
        }
        if x & 1 == 0 {
            n = (n * n) % p;
            x >>= 1;
            continue;
        } else {
            ans = (ans * n) % p;
            x -= 1;
        }
    }
}

const fn mod_inverse(n: i128, p: i128) -> i128 {
    modular_exponent(n, p - 2, p)
}

const LIMIT: i128 = 1e7 as i128;
const MULT: i128 = 1504170715041707;
const MOD: i128 = 4503599627370517;
const MOD_INV: i128 = mod_inverse(MULT, MOD);

pub fn solve() -> i128 {
    assert_eq!(3, (8912517754604 * MOD_INV) % MOD);

    let mut current = MULT;
    let mut least = i128::MAX;
    let mut answer = 0;

    while current != 1 {
        if current < least {
            least = current;
            answer += current;
            if current < LIMIT {
                break;
            }
        }
        current = (current + MULT) % MOD;
    }

    let mut small_coins = (1..current)
        .map(|x| ((MOD_INV * x) % MOD, x))
        .collect::<Vec<_>>();
    small_coins.sort_by_key(|x| x.0);

    for (_coin_index, coin) in small_coins {
        if coin < least {
            answer += coin;
            least = coin;
        }
    }

    answer
}
