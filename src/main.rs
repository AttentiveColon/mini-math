#![allow(unused_imports)]
use math::*;

fn main() {
    println!("Hi.");

    let vector = Vec3::new(0.0, 1.0, 0.0);
    let axis = Vec3::new(1.0, 0.0, 0.0);
    let rotated_vector = quaternion::rotate_on_axis(vector, -90.0, axis);

    dbg!(rotated_vector);
}
