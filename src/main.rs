extern crate nalgebra as na;

use na::{Isometry2, Point2, Vector2};
use ncollide2d::bounding_volume::{self, BoundingVolume};
use ncollide2d::shape::Ball;

fn main() {
    let ball1 = Ball::new(0.5);
    let ball2 = Ball::new(1.0);

    let ball1_pos = Isometry2::new(Vector2::y(), na::zero()); // 1.0 along the `y` axis.
    let ball2_pos = Isometry2::identity(); // Identity matrix.

    println!("{:?}", ball1_pos);

    // let p1 = Point2{1.0, 2.1};

    //   ball1_pos.transform_point(&p1);

    let aabb_ball1 = bounding_volume::aabb(&ball1, &ball1_pos);
    let aabb_ball2 = bounding_volume::aabb(&ball2, &ball2_pos);

    let bounding_aabb = aabb_ball1.merged(&aabb_ball2);

    let loose_aabb_ball2 = aabb_ball2.loosened(1.0);

    assert!(aabb_ball1.intersects(&aabb_ball2));
    assert!(bounding_aabb.contains(&aabb_ball1));
    assert!(bounding_aabb.contains(&aabb_ball2));
    assert!(!aabb_ball2.contains(&bounding_aabb));
    assert!(!aabb_ball1.contains(&bounding_aabb));
    assert!(loose_aabb_ball2.contains(&aabb_ball2));
}
