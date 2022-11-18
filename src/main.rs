mod math;
use math::vector;
use math::*;

fn main() {
    println!("Hi.");

    let vec1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let vec2 = Vec4::new(6.0, 7.0, 8.0, 9.0);
    let _res = vec1 + vec2;
    let _res = vec1.add(vec2);
    let _res = vector::add(vec1, vec2);

    let mat1 = Mat4::new(vec1, vec2, vec1, vec2);
    let mat2 = Mat4::new(vec2, vec1, vec2, vec1);
    let _res = mat1 + mat2;
    let _res = mat1.add(mat2);
    let _res = matrix::add(mat2, mat2);

    let _res = matrix::scale(mat1, 5.0);
    let _res = mat1.scale(5.0);
    let res = mat1 * 5.0;

    println!("mat4: {:?}", res);
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
    println!("mag: {}", mag);
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
fn test_vec_4() {
    use crate::math::*;

    //test new
    let vector = Vec4::new(2.0, 1.0, 3.0, 4.0);
    assert_eq!(vector, (2.0, 1.0, 3.0, 4.0).into());

    //test default
    let vector = Vec4::default();
    assert_eq!(vector, (0.0, 0.0, 0.0, 0.0).into());

    //test add
    let vector = vector.add((5.0, 4.0, 3.0, 2.0).into());
    assert_eq!(vector, (5.0, 4.0, 3.0, 2.0).into());

    //test negation
    let vector = -vector;
    assert_eq!(vector, (-5.0, -4.0, -3.0, -2.0).into());

    //test sub
    let vector = vector.sub((-10.0, -8.0, -6.0, -4.0).into());
    assert_eq!(vector, (5.0, 4.0, 3.0, 2.0).into());

    //test scale
    let vector = vector.scale(5.0);
    assert_eq!(vector, (25.0, 20.0, 15.0, 10.0).into());

    //test magnitude
    let mag = vector.magnitude();
    println!("{}", mag);
    assert!(f32::abs(mag - 36.742348) < f32::EPSILON);

    //test normalize
    let vector = vector.normalize();
    assert_eq!(vector, (0.6804138, 0.544331, 0.40824828, 0.2721655).into());
}

#[test]
fn test_vec_2_conversions() {
    use crate::math::*;

    let v3 = Vec3::new(-1.1, 2.2, -3.3);
    let v4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

    let test3: Vec2<_> = v3.into();
    let test4: Vec2<_> = v4.into();

    assert_eq!(test3, Vec2::new(-1.1, 2.2));
    assert_eq!(test4, Vec2::new(1.0, 2.0));
}

#[test]
fn test_vec_3_conversions() {
    use crate::math::*;

    let v2 = Vec2::new(-1.1, 2.2);
    let v4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

    let test3: Vec3<_> = v2.into();
    let test4: Vec3<_> = v4.into();

    assert_eq!(test3, Vec3::new(-1.1, 2.2, 0.0));
    assert_eq!(test4, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_vec_4_conversions() {
    use crate::math::*;

    let v2 = Vec2::new(-1.1, 2.2);
    let v3 = Vec3::new(1.0, 2.0, 3.0);

    let test3: Vec4<_> = v2.into();
    let test4: Vec4<_> = v3.into();

    assert_eq!(test3, Vec4::new(-1.1, 2.2, 0.0, 0.0));
    assert_eq!(test4, Vec4::new(1.0, 2.0, 3.0, 0.0));
}

#[test]
fn test_mat_2_conversions() {
    use crate::math::*;

    let m2 = Mat2 {
        x: (2.0, 3.0).into(),
        y: (4.0, 5.0).into(),
    };

    assert_eq!(m2, [[2.0, 3.0], [4.0, 5.0]].into());
    assert_eq!(m2, [2.0, 3.0, 4.0, 5.0].into());
}

#[test]
fn test_mat_3_conversions() {
    use crate::math::*;

    let m3 = Mat3 {
        x: (1.0, 2.0, 3.0).into(),
        y: (4.0, 5.0, 6.0).into(),
        z: (7.0, 8.0, 9.0).into(),
    };

    assert_eq!(
        m3,
        [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]].into()
    );
    assert_eq!(m3, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].into());
}

#[test]
fn test_mat_4_conversions() {
    use crate::math::*;

    let m4 = Mat4 {
        x: (1.0, 2.0, 3.0, 4.0).into(),
        y: (1.0, 2.0, 3.0, 4.0).into(),
        z: (1.0, 2.0, 3.0, 4.0).into(),
        w: (1.0, 2.0, 3.0, 4.0).into(),
    };

    assert_eq!(
        m4,
        [
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0]
        ]
        .into()
    );
    assert_eq!(
        m4,
        [1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0].into()
    );
}
