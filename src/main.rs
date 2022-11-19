#![allow(unused_imports)]
use math::*;

fn main() {
    println!("Hi.");

    let vec = Vec2::new(2.1, 3.1);
    println!("{vec}");

    let vec = Vec3::new(-1.0, 2.0001, 2.2);
    println!("{vec}");

    let vec = Vec4::new(1.0, 2.0, 3.0, 4.0);
    println!("{vec}");
}
