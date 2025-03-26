use super::{Point2, Point3, Vec2, Vec3};

pub trait Ray {
    type Point;
    fn at(&self, t: f32) -> Self::Point;
}

pub struct Ray2D {
    origin: Point2,
    direction: Vec2,
}

impl Ray for Ray2D {
    type Point = Point2;
    fn at(&self, t: f32) -> Self::Point {
        self.origin + t * self.direction
    }
}

pub struct Ray3D {
    origin: Point3,
    direction: Vec3,
}

impl Ray for Ray3D {
    type Point = Point3;
    fn at(&self, t: f32) -> Self::Point {
        self.origin + t * self.direction
    }
}
