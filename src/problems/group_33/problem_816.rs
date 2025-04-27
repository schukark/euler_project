pub(crate) fn solve() -> String {
    let start = 290797_i128;
    let limit = 2_000_000;
    let mut sequence = vec![start];

    for i in 1..limit * 2 {
        let next = (sequence[i - 1] * sequence[i - 1]) % 50515093;
        sequence.push(next);
    }

    let mut points: Vec<(i128, i128)> = Vec::new();
    for i in 0..limit {
        points.push((sequence[2 * i], sequence[2 * i + 1]));
    }

    points.sort_by_key(|&(x, _)| x);
    format!(
        "{:.9}",
        (find_min_dist_smart(&points, 0, points.len()) as f64).sqrt()
    )
}

fn find_min_dist_smart(points: &[(i128, i128)], start: usize, finish: usize) -> i128 {
    if finish - start <= 3 {
        return find_min_dist_naive(points, start, finish);
    }

    let mid = (start + finish) / 2;
    let left = find_min_dist_smart(points, start, mid);
    let right = find_min_dist_smart(points, mid, finish);
    let mut min_dist = left.min(right);

    let mut strip = Vec::new();
    let mid_x = points[mid].0;
    for point in points.iter().take(finish).skip(start) {
        if (point.0 - mid_x).abs() < min_dist {
            strip.push(point);
        }
    }
    strip.sort_by_key(|&(_, y)| y);

    for i in 0..strip.len() {
        for j in i + 1..strip.len().min(i + 8) {
            min_dist =
                min_dist.min((strip[i].0 - strip[j].0).pow(2) + (strip[i].1 - strip[j].1).pow(2));
        }
    }

    min_dist
}

fn find_min_dist_naive(points: &[(i128, i128)], start: usize, finish: usize) -> i128 {
    let mut min_dist = i128::MAX;
    for i in start..finish {
        for j in i + 1..finish {
            let dist = (points[i].0 - points[j].0).pow(2) + (points[i].1 - points[j].1).pow(2);
            if dist < min_dist {
                min_dist = dist;
            }
        }
    }

    min_dist
}
