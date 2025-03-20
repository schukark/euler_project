use super::point::Point;

pub struct Triangle {
    vertices: [Point; 3],
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            vertices: [Point::new(), Point::new(), Point::new()],
        }
    }

    pub fn from_vertices(vertices: &[Point; 3]) -> Triangle {
        Triangle {
            vertices: *vertices,
        }
    }

    pub fn area(&self) -> f64 {
        let vec_ab = (
            self.vertices[1].x - self.vertices[0].x,
            self.vertices[1].y - self.vertices[0].y,
        );
        let vec_ac = (
            self.vertices[2].x - self.vertices[0].x,
            self.vertices[2].y - self.vertices[0].y,
        );

        f64::abs(vec_ab.0 * vec_ac.1 - vec_ab.1 * vec_ac.0) / 2.0
    }

    pub fn contains_point(&self, point: Point) -> bool {
        let area_1 = Triangle::from_vertices(&[self.vertices[0], self.vertices[1], point]).area();
        let area_2 = Triangle::from_vertices(&[self.vertices[0], self.vertices[2], point]).area();
        let area_3 = Triangle::from_vertices(&[self.vertices[1], self.vertices[2], point]).area();

        f64::abs(area_1 + area_2 + area_3 - self.area()) < 1e-6
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::geometry::point::Point;

    use super::Triangle;

    #[test]
    fn test_triangle_contains_1() {
        let triangle = Triangle::from_vertices(&[
            Point::from_coords(-340.0, 495.0),
            Point::from_coords(-153.0, -910.0),
            Point::from_coords(835.0, -947.0),
        ]);

        assert!(triangle.contains_point(Point::new()));
    }

    #[test]
    fn test_triangle_contains_2() {
        let triangle = Triangle::from_vertices(&[
            Point::from_coords(-175.0, 41.0),
            Point::from_coords(-421.0, -714.0),
            Point::from_coords(574.0, -645.0),
        ]);

        assert!(!triangle.contains_point(Point::new()));
    }
}
