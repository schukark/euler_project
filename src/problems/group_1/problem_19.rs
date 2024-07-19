pub fn solve() -> u64 {
    let mut day = 0;
    let mut weekday = 0;
    let mut month = 0;
    let mut year = 1900;

    let month_daycount = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut sunday_count = 0;

    while year != 2001 {
        if (1901..=2000).contains(&year) && weekday == 6 && day == 0 {
            // println!("{:2}.{:2}.{:4}", day + 1, month + 1, year);
            sunday_count += 1;
        }

        day += 1;
        weekday = (weekday + 1) % 7;

        let mut cur_month_days = month_daycount[month];

        let is_leap = |x| {
            if x % 400 == 0 {
                return true;
            } else if x % 100 == 0 {
                return false;
            }
            x % 4 == 0
        };

        if month == 1 && is_leap(year) {
            cur_month_days = 29;
        }

        if day >= cur_month_days {
            day = 0;
            month += 1;
        }

        if month >= 12 {
            year += 1;
            month = 0;
        }
    }

    sunday_count
}