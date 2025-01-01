use std::ops::{Add, Neg, Sub};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

impl Sub for Point2D {
    type Output = Point2D;

    fn sub(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Point2D {
    type Output = Point2D;

    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Neg for Point2D {
    type Output = Point2D;

    fn neg(self) -> Point2D {
        Point2D {
            x: -self.x,
            y: -self.y,
        }
    }
}
