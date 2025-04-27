const LIMIT: i32 = 50;

pub fn solve() -> i128 {
    let mut ans = 0;
    for x1 in 0..=LIMIT {
        for y1 in 0..=LIMIT {
            for x2 in x1..=LIMIT {
                for y2 in 0..=LIMIT {
                    if x1 == x2 && y1 >= y2 {
                        continue;
                    }

                    if check_triangle(x1, y1, x2, y2) {
                        ans += 1;
                        // dbg!(format!("{x1}, {y1}, {x2}, {y2}"));
                    }
                }
            }
        }
    }

    ans
}

fn check_triangle(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    let a = x1.pow(2) + y1.pow(2);
    let b = x2.pow(2) + y2.pow(2);
    let c = (x2 - x1).pow(2) + (y2 - y1).pow(2);

    if a == 0 || b == 0 || c == 0 {
        return false;
    }

    let longest = a.max(b.max(c));

    match longest {
        x if x == a => longest == b + c,
        x if x == b => longest == a + c,
        x if x == c => longest == a + b,
        _ => unreachable!(),
    }
}
