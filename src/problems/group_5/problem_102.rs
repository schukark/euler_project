use std::fs;

use crate::utils::geometry::{point::Point, triangle::Triangle};

fn read_triangles_from_file(filename: &str) -> Vec<Triangle> {
    fs::read_to_string(filename)
        .expect("There is no file with such name")
        .split('\n')
        .map(|line| {
            line.split(',')
                .map(|coord| coord.parse::<f64>().unwrap())
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|coords| Point::from_coords(coords[0], coords[1]))
                .collect::<Vec<_>>()
        })
        .map(|coords| Triangle::from_vertices(&coords[..3].try_into().unwrap()))
        .collect::<Vec<_>>()
}

pub fn solve() -> i128 {
    let origin = Point::from_coords(0.0, 0.0);

    read_triangles_from_file("src/txts/prob_102.txt")
        .iter()
        .filter(|triangle| triangle.contains_point(origin))
        .count() as i128
}
