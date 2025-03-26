use std::f32::consts::PI;

use cgmath::{InnerSpace, MetricSpace};

use crate::arith::{deg, rad, rotate_vec2, rotate_vec2_deg};

use super::vec2;

// Helper function for floating point comparison with epsilon
fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < super::EPSILON
}

// Helper function for Vec2 approximate comparison
fn vec2_approx_eq(a: super::Vec2, b: super::Vec2) -> bool {
    approx_eq(a.x, b.x) && approx_eq(a.y, b.y)
}

#[test]
fn test_vec2_operations() {
    let a = vec2(1.0, 1.0);
    let b = vec2(5.0, 2.0);

    let v: f32 = 29.0;
    assert!(approx_eq(b.magnitude(), v.sqrt()));

    let normalized = a.normalize();
    let expected = vec2(1.0, 1.0) / a.magnitude();
    assert!(vec2_approx_eq(normalized, expected));

    assert_eq!(a + b, vec2(6.0, 3.0));
    assert_eq!(a - b, vec2(-4.0, -1.0));
    assert_eq!(a * 2.0, vec2(2.0, 2.0));
    assert_eq!(a.dot(b), 7.0);
    assert_eq!(a.distance2(b), 17.0);

    let a = vec2(1.0, 0.0);
    // Using approx_eq for floating point comparisons
    assert!(approx_eq(rotate_vec2_deg(a, deg(90.0)).magnitude2(), 1.0));

    let rotated = rotate_vec2(a, rad(PI / 2.0));
    let expected = vec2(0.0, 1.0);
    assert!(
        vec2_approx_eq(rotated, expected),
        "Expected: ({}, {}), Got: ({}, {})",
        expected.x,
        expected.y,
        rotated.x,
        rotated.y
    );
}
