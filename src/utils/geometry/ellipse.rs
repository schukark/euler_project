use std::fmt::Display;

use super::{line::Line, point::Point};

/// The ellipse is represented by an equation: (x - x_center) ^ 2 / a ^ 2 + (y - y_center) ^ 2 / b ^ 2 = 1

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    center: Point,
    a: f64,
    b: f64,
}

impl Ellipse {
    pub fn new() -> Ellipse {
        Ellipse {
            center: Point::default(),
            a: 0.0,
            b: 0.0,
        }
    }

    pub fn tangent_at_point(&self, point: Point) -> Line {
        // slope is m = -a/b
        // l: y = -a/b * x + q => by = -ax + q => q = ax + by

        Line::from_coefs(self.a, self.b, self.a * point.x + self.b * point.y)
    }

    pub fn intersect_line(&self, line: Line) -> Vec<Point> {
        // ax + by + c = 0
        // (x-x_0)^2 / a^2 + ()
        unimplemented!()
    }
}

impl Default for Ellipse {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Ellipse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(x {:+})^2 / {}^2 + (y {:+})^2 / {}^2",
            -self.center.x, self.a, -self.center.y, self.b
        )
    }
}
