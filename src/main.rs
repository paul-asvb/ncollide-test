extern crate nalgebra as na;

use na::{Isometry2, Point2, Translation2, Vector2};
use ncollide2d::bounding_volume::{self, BoundingVolume};
use ncollide2d::shape::Ball;

fn main() {
    let ball1 = Ball::new(0.5);
    let ball2 = Ball::new(1.0);

    let mut ball1_pos = Isometry2::new(Vector2::y(), na::zero()); // 1.0 along the `y` axis.
    let ball2_pos = Isometry2::identity(); // Identity matrix.

    println!("{:?}", ball1_pos);

    // let p1 = Point2{1.0, 2.1};

    //   ball1_pos.transform_point(&p1);

    let aabb_ball1 = bounding_volume::aabb(&ball1, &ball1_pos);
    let aabb_ball2 = bounding_volume::aabb(&ball2, &ball2_pos);

    let trans = Translation2::new(10.0, 20.0);

    ball1_pos.translation = trans;

    println!("{:?}", ball1_pos);

    println!("{:?}", aabb_ball1.intersects(&aabb_ball2));
}
