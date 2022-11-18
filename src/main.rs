mod math;

fn main() {
    println!("Hi.");
}

#[test]
fn test_point_2() {
    use crate::math::*;

    //test new
    let point = Point2::new(2.0, 1.0);
    assert_eq!(point, (2.0, 1.0).into());

    //test default
    let point = Point2::default();
    assert_eq!(point, (0.0, 0.0).into());

    //test add
    let point = point.add((1.0, 5.0).into());
    assert_eq!(point, (1.0, 5.0).into());

    //test negation
    let point = -point;
    assert_eq!(point, (-1.0, -5.0).into());
}

#[test]
fn test_point_3() {
    use crate::math::*;

    //test new
    let point = Point3::new(3.0, 2.0, 1.0);
    assert_eq!(point, (3.0, 2.0, 1.0).into());

    //test default
    let point = Point3::default();
    assert_eq!(point, (0.0, 0.0, 0.0).into());

    //test add
    let point = point.add((5.0, 4.0, 3.0).into());
    assert_eq!(point, (5.0, 4.0, 3.0).into());

    //test negation
    let point = -point;
    assert_eq!(point, (-5.0, -4.0, -3.0).into());
}

#[test]
fn test_vec_2() {
    use crate::math::*;

    //test new
    let vector = Vec2::new(2.0, 1.0);
    assert_eq!(vector, (2.0, 1.0).into());

    //test default
    let vector = Vec2::default();
    assert_eq!(vector, (0.0, 0.0).into());

    //test add
    let vector = vector.add((5.0, 4.0).into());
    assert_eq!(vector, (5.0, 4.0).into());

    //test negation
    let vector = -vector;
    assert_eq!(vector, (-5.0, -4.0).into());

    //test sub
    let vector = vector.sub((-10.0, -8.0).into());
    assert_eq!(vector, (5.0, 4.0).into());

    //test scale
    let vector = vector.scale(5.0);
    assert_eq!(vector, (25.0, 20.0).into());

    //test magnitude
    let mag = vector.magnitude();
    assert!(f32::abs(mag - 32.01562) < f32::EPSILON);

    //test normalize
    let vector = vector.normalize();
    assert_eq!(vector, (0.7808688, 0.62469506).into());
}

#[test]
fn test_vec_3() {
    use crate::math::*;

    //test new
    let vector = Vec3::new(2.0, 1.0, 3.0);
    assert_eq!(vector, (2.0, 1.0, 3.0).into());

    //test default
    let vector = Vec3::default();
    assert_eq!(vector, (0.0, 0.0, 0.0).into());

    //test add
    let vector = vector.add((5.0, 4.0, 3.0).into());
    assert_eq!(vector, (5.0, 4.0, 3.0).into());

    //test negation
    let vector = -vector;
    assert_eq!(vector, (-5.0, -4.0, -3.0).into());

    //test sub
    let vector = vector.sub((-10.0, -8.0, -6.0).into());
    assert_eq!(vector, (5.0, 4.0, 3.0).into());

    //test scale
    let vector = vector.scale(5.0);
    assert_eq!(vector, (25.0, 20.0, 15.0).into());

    //test magnitude
    let mag = vector.magnitude();
    assert!(f32::abs(mag - 35.35534) < f32::EPSILON);

    //test normalize
    let vector = vector.normalize();
    assert_eq!(vector, (0.70710677, 0.56568545, 0.42426407).into());
}

#[test]
fn test_point_2_conversions() {
    use crate::math::*;

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
    use crate::math::*;

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
    use crate::math::*;

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
    use crate::math::*;

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
