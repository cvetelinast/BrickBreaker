use brick_breaker::*;
use crate::math::{circle_collides_rect, Collision};

#[test]
fn test_basic() {

}

#[test]
fn test_math_collisions() {
    // cx: 1.0, cy: 1.0, radius: 2.0, rx: 0.0, ry: 0.0, rw: 4.0, rh: 2.0
    let result = circle_collides_rect(1.0, 1.0, 1.0, 2.0, 0.0, 4.0, 2.0);
    assert!(matches!(result, Collision::Left));

    let result = circle_collides_rect(4.0, 3.0, 1.0, 2.0, 0.0, 4.0, 2.0);
    assert!(matches!(result, Collision::Bottom));

    let result = circle_collides_rect(4.0, 3.0, 0.5, 2.0, 0.0, 4.0, 2.0);
    assert!(matches!(result, Collision::None));

    let result = circle_collides_rect(6.0, 3.0, 1.0, 2.0, 0.0, 2.0, 2.0);
    assert!(matches!(result, Collision::None));
}