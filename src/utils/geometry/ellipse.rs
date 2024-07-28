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
        // (x-x_0)^2 / a^2 + ((-c-ax)/b-y_0)^2/b^2=1
        // (x^2 - 2x*x_0 + x_0^2) / a^2 + (c^2 / b^2 + 2acx / b^2 + a^2x^2 / b^2 - 2y_0 * (-c - ax) / b + y_0^2)^2 / b^2 = 1
        //
        // x^2 (a^2 / b^4 + 1 / a^2) + x * (-2x_0 / a^2 + 2ac / b^4 + 2ay_0 / b^3) +
        // + (x_0^2 / a^2 + c^2 / b^4 + 2cy_0 / b^3 + y_0^2 / b^2) = 0 | * a^2b^4

        let a = line.a;
        let b = line.b;
        let c = line.c;
        let x_0 = self.center.x;
        let y_0 = self.center.y;

        let a_coef = a.powf(4.0) + b.powf(4.0);
        let b_coef = -2.0 * x_0 * b.powf(4.0) + 2.0 * a.powf(3.0) * c + 2.0 * a.powf(3.0) * b * y_0;
        let c_coef = x_0.powf(2.0) * b.powf(4.0)
            + (a * c).powf(2.0)
            + 2.0 * y_0 * a.powf(2.0) * b * c
            + (a * b * y_0).powf(2.0);

        let (a, b, c) = (a_coef, b_coef, c_coef);

        if f64::abs(a) < 1e-6 {
            let x = -c / b;
            let y = (-c - a * x) / b;

            vec![Point::from_coords(x, y)]
        } else {
            let discriminant = b.powf(2.0) - 4.0 * a * c;
            let x_values = [
                (-b + discriminant.sqrt()) / 2.0 / a,
                (-b - discriminant.sqrt()) / 2.0 / a,
            ];
            let y_values = x_values
                .iter()
                .map(|x| (-c - a * x) / b)
                .collect::<Vec<_>>();

            vec![
                Point::from_coords(x_values[0], y_values[0]),
                Point::from_coords(x_values[1], y_values[1]),
            ]
        }
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
