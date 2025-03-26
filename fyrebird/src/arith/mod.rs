pub mod ray;

#[cfg(test)]
mod tests;

use cgmath::{InnerSpace, Matrix4, Rotation3, Vector2, Vector3};
use num_traits::{Float, Num};

pub const EPSILON: f32 = 1e-6;

pub type Deg = cgmath::Deg<f32>;
pub type Rad = cgmath::Rad<f32>;

/// Create a Deg angle from degrees
pub const fn deg(degrees: f32) -> Deg {
    cgmath::Deg(degrees)
}

/// Create a Rad angle from radians
pub const fn rad(radians: f32) -> Rad {
    cgmath::Rad(radians)
}

pub type Vec2 = Vector2<f32>;
pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

pub type Vec3 = Vector3<f32>;
pub const fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

pub type Mat4 = Matrix4<f32>;

pub type Vec2Int = Vector2<i32>;
pub const fn vec2_int(x: i32, y: i32) -> Vec2Int {
    Vec2Int { x, y }
}
pub type Vec3Int = Vector3<i32>;
pub const fn vec3_int(x: i32, y: i32, z: i32) -> Vec3Int {
    Vec3Int { x, y, z }
}

pub type Point2 = cgmath::Point2<f32>;
pub type Point3 = cgmath::Point3<f32>;

pub type Point2Int = cgmath::Point2<i32>;
pub type Point3Int = cgmath::Point3<i32>;

pub type Quaternion = cgmath::Quaternion<f32>;
pub type QuaternionInt = cgmath::Quaternion<i32>;

// Useful quaternion functions
pub fn quat_from_axis_angle(axis: Vec3, angle: Rad) -> Quaternion {
    cgmath::Quaternion::from_axis_angle(axis, angle)
}

pub fn quat_from_euler(pitch: Rad, yaw: Rad, roll: Rad) -> Quaternion {
    let (sp, cp) = pitch.0.sin_cos();
    let (sy, cy) = yaw.0.sin_cos();
    let (sr, cr) = roll.0.sin_cos();

    Quaternion::new(
        cr * cp * cy + sr * sp * sy,
        sr * cp * cy - cr * sp * sy,
        cr * sp * cy + sr * cp * sy,
        cr * cp * sy - sr * sp * cy,
    )
}

// Vector rotation functions
pub fn rotate_vec2(v: Vec2, angle: Rad) -> Vec2 {
    let (sin, cos): (f32, f32) = angle.0.sin_cos();
    Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
}

pub fn rotate_vec2_deg(v: Vec2, angle: Deg) -> Vec2 {
    let (sin, cos) = angle.0.sin_cos();
    Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
}

pub fn rotate_vec3(v: Vec3, rotation: Quaternion) -> Vec3 {
    rotation * v
}

pub fn rotate_around_axis(v: Vec3, axis: Vec3, angle: Rad) -> Vec3 {
    let q = quat_from_axis_angle(axis.normalize(), angle);
    rotate_vec3(v, q)
}

// Generic number interval implementation
pub struct Interval<T>
where
    T: Num + PartialOrd + Copy,
{
    pub min: T,
    pub max: T,
}

impl<T> Interval<T>
where
    T: Num + PartialOrd + Copy,
{
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, value: T) -> bool {
        value >= self.min && value <= self.max
    }

    pub fn clamp(&self, value: T) -> T {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }
}

// For floating point intervals, add extra functionality
impl<T> Interval<T>
where
    T: Float,
{
    pub fn lerp(&self, t: T) -> T {
        self.min + (self.max - self.min) * t
    }

    pub fn inverse_lerp(&self, value: T) -> T {
        if self.min == self.max {
            T::zero()
        } else {
            (value - self.min) / (self.max - self.min)
        }
    }

    pub fn remap(&self, value: T, target_min: T, target_max: T) -> T {
        let normalized = self.inverse_lerp(value);
        target_min + normalized * (target_max - target_min)
    }

    pub fn size(&self) -> T {
        self.max - self.min
    }
}
