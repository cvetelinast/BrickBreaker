#[derive(Debug)]
pub enum Collision {
    Left,
    Top,
    Right,
    Bottom,
    None,
}

// CIRCLE AND RECTANGLE COLLISION
pub fn circle_collides_rect(
    cx: f32,
    cy: f32,
    radius: f32,
    rx: f32,
    ry: f32,
    rw: f32,
    rh: f32,
) -> Collision {
    let mut horizontal_collision = Collision::None;
    let mut vertical_collision = Collision::None;

    // temporary variables to set edges for testing
    let mut test_x = cx;
    let mut test_y = cy;
    // which edge is closest?
    if cx < rx {
        test_x = rx; // test left edge
        horizontal_collision = Collision::Left;
    } else if cx > rx + rw {
        test_x = rx + rw; // right edge
        horizontal_collision = Collision::Right;
    }

    if cy < ry {
        test_y = ry; // top edge
        vertical_collision = Collision::Top;
    } else if cy > ry + rh {
        test_y = ry + rh; // bottom edge
        vertical_collision = Collision::Bottom;
    }
    // get distance from closest edges
    let dist_x = cx - test_x;
    let dist_y = cy - test_y;
    let distance = ((dist_x * dist_x) + (dist_y * dist_y)).sqrt();

    if distance <= radius {
        if dist_x.abs() >= dist_y.abs() {
            return horizontal_collision;
        } else {
            return vertical_collision;
        }
    } else {
        return Collision::None;
    }
}
