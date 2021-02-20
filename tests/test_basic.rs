use crate::gameplay_entities::{Ball, BallState, BricksWall, Size, Skateboard};
use crate::math::{circle_collides_rect, Collision};
use brick_breaker::*;
use ggez::mint::Point2;

#[test]
fn test_math_collisions() {
    let result = circle_collides_rect(1.0, 1.0, 1.0, 2.0, 0.0, 4.0, 2.0);
    assert!(matches!(result, Collision::Left));

    let result = circle_collides_rect(4.0, 3.0, 1.0, 2.0, 0.0, 4.0, 2.0);
    assert!(matches!(result, Collision::Bottom));

    let result = circle_collides_rect(4.0, 3.0, 0.5, 2.0, 0.0, 4.0, 2.0);
    assert!(matches!(result, Collision::None));

    let result = circle_collides_rect(6.0, 3.0, 1.0, 2.0, 0.0, 2.0, 2.0);
    assert!(matches!(result, Collision::None));

    let result = circle_collides_rect(1.0, 1.0, 1.0, 2.0, 0.0, 1.0, 1.0);
    assert!(matches!(result, Collision::Left));

    let result = circle_collides_rect(1.0, 1.0, 1.0, 1.5, 1.0, 1.0, 1.0);
    assert!(matches!(result, Collision::Left));
}

#[test]
fn test_ball_initialization() {
    let actual_ball = Ball::new(200.0, 100.0, 60.0, 10.0, 10.0);
    assert!(matches!(actual_ball.state, BallState::Flying));
    assert_eq!(actual_ball.pos, Point2 { x: 95.0, y: 30.0 });
    assert_eq!(actual_ball.direction, Point2 { x: 1.0, y: 1.0 });
    assert_eq!(actual_ball.radius, 5.0);

    let actual_ball_2 = Ball::new(856.0, 3456.0, 1008.0, 16.0, 34.0);
    assert!(matches!(actual_ball_2.state, BallState::Flying));
    assert_eq!(
        actual_ball_2.pos,
        Point2 {
            x: 420.0,
            y: 2414.0
        }
    );
    assert_eq!(actual_ball_2.direction, Point2 { x: 1.0, y: 1.0 });
    assert_eq!(actual_ball_2.radius, 8.0);
}

#[test]
fn test_bricks_wall_initialization() {
    let wall = BricksWall::new(100.0, 400.0, 20.0, 40.0);
    assert_eq!(wall.bricks.len(), 6);

    let wall_2 = BricksWall::new(444.0, 324.0, 2.0, 16.0);
    assert_eq!(wall_2.bricks.len(), 88);

    let wall_3 = BricksWall::new(18684.0, 556.0, 283.0, 162.0);
    assert_eq!(wall_3.bricks.len(), 102);

    let wall_4 = BricksWall::new(0.0, 0.0, 10.0, 15.0);
    assert_eq!(wall_4.bricks.len(), 0);

    let wall_5 = BricksWall::new(100.0, 0.0, 10.0, 15.0);
    assert_eq!(wall_5.bricks.len(), 0);
}

#[test]
fn test_skateboard_initialization() {
    let skate_1 = Skateboard::new(
        Size {
            width: 20.0,
            height: 30.0,
        },
        100.0,
        400.0,
    );

    assert_eq!(skate_1.pos, Point2 { x: 190.0, y: 70.0 });

    let skate_2 = Skateboard::new(
        Size {
            width: 50.0,
            height: 16.0,
        },
        200.0,
        10000.0,
    );

    assert_eq!(
        skate_2.pos,
        Point2 {
            x: 4975.0,
            y: 184.0
        }
    );

    let skate_3 = Skateboard::new(
        Size {
            width: 1.4,
            height: 12.0,
        },
        18.0,
        25.0,
    );

    assert_eq!(skate_3.pos, Point2 { x: 11.8, y: 6.0 });
}
