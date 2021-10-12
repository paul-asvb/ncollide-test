extern crate nalgebra as na;

use na::{Isometry, Isometry2, Point2, Translation2, Vector2};
use ncollide2d::bounding_volume::{self, BoundingVolume};
use ncollide2d::query;
use ncollide2d::shape::{Ball, Compound, Cuboid, ShapeHandle};

fn main() {
    let ball1_1 = Ball::new(1.0);
    let ball1_2 = Ball::new(1.0);
    let ball1_3 = Ball::new(1.0);

    let ball2 = Ball::new(1.0);

    let ball1_1_pos = Isometry2::new(Vector2::new(0.0, 0.0), na::zero());
    let ball1_2_pos = Isometry2::new(Vector2::new(0.0, 1.0), na::zero());
    let ball1_3_pos = Isometry2::new(Vector2::new(0.0, 2.0), na::zero());
    let ball2_pos = Isometry2::new(Vector2::new(0.0, 4.0), na::zero());

    // // 1) Initialize the shape list.
    let mut shapes = Vec::new();
    let ball1_1_handle = ShapeHandle::new(ball1_1);
    let ball1_2_handle = ShapeHandle::new(ball1_2);
    let ball1_3_handle = ShapeHandle::new(ball1_3);
    // let vertical_box = ShapeHandle::new(Cuboid::new(Vector2::new(0.25f32, 1.5)));

    shapes.push((ball1_1_pos, ball1_1_handle));
    shapes.push((ball1_2_pos, ball1_2_handle));
    shapes.push((ball1_3_pos, ball1_3_handle));
    //shapes.push((ball1_2_pos, ball1_2));
    //shapes.push((ball1_3_pos, ball1_3));

    // // 2) Create the compound shape.
    let compound = Compound::new(shapes);
    let compound_pos = Isometry2::new(Vector2::new(0.0, 0.0), na::zero());

    let should_have_contact = query::contact(&compound_pos, &compound, &ball2_pos, &ball2, 0.0);
    println!("{:?}", should_have_contact);

    // println!("{:?}", too_far);

    // println!("{:?}", ball1_pos);

    // // let p1 = Point2{1.0, 2.1};

    // //   ball1_pos.transform_point(&p1);

    // let aabb_ball1 = bounding_volume::aabb(&ball1, &ball1_pos);
    // let aabb_ball2 = bounding_volume::aabb(&ball2, &ball2_pos);

    // let trans = Translation2::new(10.0, 20.0);

    // ball1_pos.translation = trans;

    // println!("{:?}", ball1_pos);

    // println!("{:?}", aabb_ball1.intersects(&aabb_ball2));

    // let cuboid = Cuboid::new(Vector2::new(1.0, 1.0));
    // let ball = Ball::new(1.0);
    // let prediction = 1.0;

    // let cuboid_pos = na::one();
    // let ball_pos_penetrating = Isometry2::new(Vector2::new(1.0, 1.0), na::zero());
    // //let ball_pos_in_prediction = Isometry2::new(Vector2::new(2.0, 2.0), na::zero());
    // let ball_pos_too_far = Isometry2::new(Vector2::new(3.0, 3.0), na::zero());

    // let ctct_penetrating = query::contact(
    //     &ball_pos_penetrating,
    //     &ball,
    //     &cuboid_pos,
    //     &cuboid,
    //     prediction,
    // );

    // println!("{:?}", ctct_penetrating);

    // let too_far = query::contact(
    //     &ball_pos_penetrating,
    //     &ball,
    //     &cuboid_pos,
    //     &cuboid,
    //     prediction,
    // );

    // println!("{:?}", too_far);
}
