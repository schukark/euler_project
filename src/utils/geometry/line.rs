use std::fmt::Display;

use super::point::Point;

/// Line is represented as an equation of the form ax + by + c = 0

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Line {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Line {
    pub fn new() -> Line {
        Line {
            a: 0.0,
            b: 0.0,
            c: 0.0,
        }
    }

    pub fn from_coefs(a: f64, b: f64, c: f64) -> Line {
        Line { a, b, c }
    }

    pub fn from_2_points(first: Point, second: Point) -> Line {
        // ax_0 + by_0 + c = 0
        // ax_1 + by_1 + c = 0
        // a(x_0 - x_1) + b(y_0 - y_1) = 0

        let a = first.x - second.x;
        let b = second.y - first.y;
        let c = -(a * first.x + b * first.y);

        Line { a, b, c }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x {:+}y {:+} = 0", self.a, self.b, self.c)
    }
}
