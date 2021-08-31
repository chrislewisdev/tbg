use std::ops;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

// TODO: Add point multiplication
