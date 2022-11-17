mod math;

use math::*;

fn main() {
    dbg!(Point2::<f32>::default());

    let point = Point2::new(5.0, 3.0);
    let direction = Vec3::new(6.0, 4.0, 2.0);
    let point = point.add((-direction).into());

    println!("{:?}", -point);

    let vector = Vec2::new(5.5, 6.6);
    println!("{:?}", vector.magnitude());

    let vector = Vec2::new(1000.0, 689.0);
    println!("{:?}", vector.normalize());
}

#[test]
fn test_point_2_conversions() {
    let p3 = Point3::new(1., 2., 3.);
    let v2 = Vec2::new(99.9, 72.1);
    let v3 = Vec3::new(-1.1, 2.2, -3.3);

    let test1: Point2<_> = p3.into();
    let test2: Point2<_> = v2.into();
    let test3: Point2<_> = v3.into();

    assert_eq!(test1, (1., 2.).into());
    assert_eq!(test2, [99.9, 72.1].into());
    assert_eq!(test3, Point2::new(-1.1, 2.2));
}

#[test]
fn test_point_3_conversions() {
    let p2 = Point2::new(1., 2.);
    let v2 = Vec2::new(99.9, 72.1);
    let v3 = Vec3::new(-1.1, 2.2, -3.3);

    let test1: Point3<_> = p2.into();
    let test2: Point3<_> = v2.into();
    let test3: Point3<_> = v3.into();

    assert_eq!(test1, (1., 2., 0.).into());
    assert_eq!(test2, [99.9, 72.1, 0.0].into());
    assert_eq!(test3, Point3::new(-1.1, 2.2, -3.3));
}

#[test]
fn test_vec_2_conversions() {
    let p2 = Point2::new(1., 2.);
    let p3 = Point3::new(99.9, 72.1, 1.0);
    let v3 = Vec3::new(-1.1, 2.2, -3.3);

    let test1: Vec2<_> = p2.into();
    let test2: Vec2<_> = p3.into();
    let test3: Vec2<_> = v3.into();

    assert_eq!(test1, (1., 2.).into());
    assert_eq!(test2, [99.9, 72.1].into());
    assert_eq!(test3, Vec2::new(-1.1, 2.2));
}

#[test]
fn test_vec_3_conversions() {
    let p2 = Point2::new(1., 2.);
    let p3 = Point3::new(99.9, 72.1, 7.2);
    let v2 = Vec2::new(-1.1, 2.2);

    let test1: Vec3<_> = p2.into();
    let test2: Vec3<_> = p3.into();
    let test3: Vec3<_> = v2.into();

    assert_eq!(test1, (1., 2., 0.).into());
    assert_eq!(test2, [99.9, 72.1, 7.2].into());
    assert_eq!(test3, Vec3::new(-1.1, 2.2, 0.0));
}
