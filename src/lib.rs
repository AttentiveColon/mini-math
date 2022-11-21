pub mod base;
pub mod matrix;
pub mod vector;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Re-Exports
///////////////////////////////////////////////////////////////////////////////////////////////////

pub use base::Float;
pub use matrix::{Mat2, Mat3, Mat4, Matrix, MatrixOps};
pub use vector::{Vec2, Vec3, Vec4, Vector, VectorOps};

///////////////////////////////////////////////////////////////////////////////////////////////////
// Tests
///////////////////////////////////////////////////////////////////////////////////////////////////

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_statics() {
        let vec2 = Vec2::new(1.0, 2.0);
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let vec4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        //test add
        let res = Vector::add(vec2, vec2);
        assert_eq!(res, Vec2::new(2.0, 4.0));
        let res = Vector::add(vec3, vec3);
        assert_eq!(res, Vec3::new(2.0, 4.0, 6.0));
        let res = Vector::add(vec4, vec4);
        assert_eq!(res, Vec4::new(2.0, 4.0, 6.0, 8.0));

        //test sub
        let res = Vector::sub(vec2, vec2);
        assert_eq!(res, Vec2::default());
        let res = Vector::sub(vec3, vec3);
        assert_eq!(res, Vec3::default());
        let res = Vector::sub(vec4, vec4);
        assert_eq!(res, Vec4::default());

        //test scale
        let res = Vector::scale(vec2, 2.0);
        assert_eq!(res, Vec2::new(2.0, 4.0));
        let res = Vector::scale(vec3, 2.0);
        assert_eq!(res, Vec3::new(2.0, 4.0, 6.0));
        let res = Vector::scale(vec4, 2.0);
        assert_eq!(res, Vec4::new(2.0, 4.0, 6.0, 8.0));

        //test magnitude
        let res = Vector::magnitude(vec2);
        assert!(f64::abs(res - 2.23606797749979) < f64::EPSILON);
        let res = Vector::magnitude(vec3);
        assert!(f64::abs(res - 3.7416573867739413) < f64::EPSILON);
        let res = Vector::magnitude(vec4);
        assert!(f64::abs(res - 5.477225575051661) < f64::EPSILON);

        //test square magnitude
        let res = Vector::sq_magnitude(vec2);
        assert_eq!(res, 5.0);
        let res = Vector::sq_magnitude(vec3);
        assert_eq!(res, 14.0);
        let res = Vector::sq_magnitude(vec4);
        assert_eq!(res, 30.0);

        //test normalize
        let res = Vector::normalize(vec2);
        assert_eq!(res, (0.4472135954999579, 0.8944271909999159).into());
        let res = Vector::normalize(vec3);
        assert_eq!(res, (0.2672612419124244, 0.5345224838248488, 0.8017837257372732).into());
        let res = Vector::normalize(vec4);
        assert_eq!(res,(0.18257418583505536, 0.3651483716701107, 0.5477225575051661, 0.7302967433402214).into());

        //test distance
        let from = Vec2::new(1.0, 1.0);
        let to = Vec2::zeroed();
        let res = Vector::distance(from, to);
        assert_eq!(res, (-1.0, -1.0).into());
        let from = Vec3::zeroed();
        let to = Vec3::new(4.0, 1.0, 2.0);
        let res = Vector::distance(from, to);
        assert_eq!(res, (4.0, 1.0, 2.0).into());
        let from = Vec4::new(1.0, 1.0, 2.0, 2.0);
        let to = Vec4::new(4.0, 1.0, 3.0, 3.0);
        let res = Vector::distance(from, to);
        assert_eq!(res, (3.0, 0.0, 1.0, 1.0).into());

        //test dot
        let vec1 = Vec2::new(1.0, 2.0);
        let vec2 = Vec2::new(3.0, 4.0);
        let res = Vector::dot(vec1, vec2);
        assert_eq!(res, 11.0);
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(3.0, 4.0, 5.0);
        let res = Vector::dot(vec1, vec2);
        assert_eq!(res, 26.0);
        let vec1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let vec2 = Vec4::new(3.0, 4.0, 5.0, 6.0);
        let res = Vector::dot(vec1, vec2);
        assert_eq!(res, 50.0);

        //test angle
        let vec1 = Vec2::new(1.0, 0.0);
        let vec2 = Vec2::new(0.0, 1.0);
        let res = Vector::angle(vec1, vec2).to_degrees();
        assert!(f64::abs(res - 90.0) < 0.001);
        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(-3.0, 0.0, 0.0);
        let res = Vector::angle(vec1, vec2).to_degrees();
        assert!(f64::abs(res - 180.0) < 0.001);
        let vec1 = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let vec2 = Vec4::new(0.0, 4.0, 0.0, 0.0);
        let res = Vector::angle(vec1, vec2).to_degrees();
        assert!(f64::abs(res - 90.0) < 0.001);

        //test lerp
        let vec1 = Vec2::new(0.0, 0.0);
        let vec2 = Vec2::new(4.0, 4.0);
        assert_eq!(Vector::lerp(vec1, vec2, 0.0), vec1);
        assert_eq!(Vector::lerp(vec1, vec2, 0.25), Vec2::new(1.0, 1.0));
        assert_eq!(Vector::lerp(vec1, vec2, 0.50), Vec2::new(2.0, 2.0));
        assert_eq!(Vector::lerp(vec1, vec2, 0.75), Vec2::new(3.0, 3.0));
        assert_eq!(Vector::lerp(vec1, vec2, 1.0), vec2);
        let vec1 = Vec3::new(0.0, 0.0, 0.0);
        let vec2 = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(Vector::lerp(vec1, vec2, 0.0), vec1);
        assert_eq!(Vector::lerp(vec1, vec2, 0.25), Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(Vector::lerp(vec1, vec2, 0.50), Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(Vector::lerp(vec1, vec2, 0.75), Vec3::new(3.0, 3.0, 3.0));
        assert_eq!(Vector::lerp(vec1, vec2, 1.0), vec2);
        let vec1 = Vec4::new(0.0, 0.0, 0.0, 0.0);
        let vec2 = Vec4::new(4.0, 4.0, 4.0, 4.0);
        assert_eq!(Vector::lerp(vec1, vec2, 0.0), vec1);
        assert_eq!(Vector::lerp(vec1, vec2, 0.25), Vec4::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(Vector::lerp(vec1, vec2, 0.50), Vec4::new(2.0, 2.0, 2.0, 2.0));
        assert_eq!(Vector::lerp(vec1, vec2, 0.75), Vec4::new(3.0, 3.0, 3.0, 3.0));
        assert_eq!(Vector::lerp(vec1, vec2, 1.0), vec2);

        //test nlerp
        let vec1 = Vec2::new(-200.0, 0.0);
        let vec2 = Vec2::new(0.0, 200.0);
        assert_eq!(Vector::nlerp(vec1, vec2, 0.0), vec1.normalize());
        assert_eq!( Vector::nlerp(vec1, vec2, 0.5), (-0.7071067811865475, 0.7071067811865475).into());
        assert_eq!(Vector::nlerp(vec1, vec2, 1.0), vec2.normalize());
        let vec1 = Vec3::new(-200.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 200.0, 0.0);
        assert_eq!(Vector::nlerp(vec1, vec2, 0.0), vec1.normalize());
        assert_eq!(Vector::nlerp(vec1, vec2, 0.5),(-0.7071067811865475, 0.7071067811865475, 0.0).into());
        assert_eq!(Vector::nlerp(vec1, vec2, 1.0), vec2.normalize());
        let vec1 = Vec4::new(-200.0, 0.0, 0.0, 0.0);
        let vec2 = Vec4::new(0.0, 200.0, 0.0, 0.0);
        assert_eq!(Vector::nlerp(vec1, vec2, 0.0), vec1.normalize());
        assert_eq!(Vector::nlerp(vec1, vec2, 0.5),(-0.7071067811865475, 0.7071067811865475, 0.0, 0.0).into());
        assert_eq!(Vector::nlerp(vec1, vec2, 1.0), vec2.normalize());

        //test slerp
        let vec1 = Vec2::new(1.0, 0.0);
        let vec2 = Vec2::new(0.0, 1.0);
        assert!(Vector::slerp(vec1, vec2, 0.0).approx_eq(vec1, f32::EPSILON));
        assert!(Vector::slerp(vec1, vec2, 0.25).approx_eq(Vec2::new(0.92387, 0.38268), 0.0001));
        assert!(Vector::slerp(vec1, vec2, 0.5).approx_eq(Vec2::new(0.70710, 0.70710), 0.0001));
        assert!(Vector::slerp(vec1, vec2, 0.75).approx_eq(Vec2::new(0.38268, 0.92387), 0.0001));
        assert!(vec1.slerp(vec2, 1.0).approx_eq(vec2, f32::EPSILON));
        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 1.0, 0.0);
        assert!(Vector::slerp(vec1, vec2, 0.0).approx_eq(vec1, f32::EPSILON));
        assert!(Vector::slerp(vec1, vec2, 0.25).approx_eq(Vec3::new(0.92387, 0.38268, 0.0), 0.0001));
        assert!(Vector::slerp(vec1, vec2, 0.5).approx_eq(Vec3::new(0.70710, 0.70710, 0.0), 0.0001));
        assert!(Vector::slerp(vec1, vec2, 0.75).approx_eq(Vec3::new(0.38268, 0.92387, 0.0), 0.0001));
        assert!(Vector::slerp(vec1, vec2, 1.0).approx_eq(vec2, f32::EPSILON));
        let vec1 = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let vec2 = Vec4::new(0.0, 1.0, 0.0, 0.0);
        assert!(Vector::slerp(vec1, vec2, 0.0).approx_eq(vec1, f32::EPSILON));
        assert!(Vector::slerp(vec1, vec2, 0.25).approx_eq(Vec4::new(0.92387, 0.38268, 0.0, 0.0), 0.0001));
        assert!(Vector::slerp(vec1, vec2, 0.5).approx_eq(Vec4::new(0.70710, 0.70710, 0.0, 0.0), 0.0001));
        assert!(Vector::slerp(vec1, vec2, 0.75).approx_eq(Vec4::new(0.38268, 0.92387, 0.0, 0.0), 0.0001));
        assert!(Vector::slerp(vec1, vec2, 1.0).approx_eq(vec2, f32::EPSILON));
    }

    #[test]
    fn test_vec_2() {
        //test new
        let vector = Vec2::new(2.0, 1.0);
        assert_eq!(vector, (2.0, 1.0).into());

        //test unit_x
        let vector = Vec2::unit_x();
        assert_eq!(vector, (1.0, 0.0).into());

        //test unit_y
        let vector = Vec2::unit_y();
        assert_eq!(vector, (0.0, 1.0).into());

        //test zeroed
        assert_eq!(Vec2::zeroed(), (0.0, 0.0).into());

        //test add
        let vector = Vec2::new(0.0, 1.0);
        let vector = vector.add((5.0, 4.0).into());
        assert_eq!(vector, (5.0, 5.0).into());

        //test sub
        let vector = vector.sub((-5.0, -5.0).into());
        assert_eq!(vector, (10.0, 10.0).into());

        //test scale
        let vector = vector.scale(5.0);
        assert_eq!(vector, (50.0, 50.0).into());

        //test magnitude
        let mag = vector.magnitude();
        assert!(f32::abs(mag - 70.71068) < f32::EPSILON);

        //test sq_magnitude
        let sq_mag = vector.sq_magnitude();
        assert_eq!(sq_mag, 5000.0);

        //test normalize
        let vector = vector.normalize();
        assert_eq!(vector, (0.7071067811865475, 0.7071067811865475).into());

        //test distance
        let from = Vec2::zeroed();
        let to = Vec2::new(1.0, 2.0);
        let vector = from.distance(to);
        assert_eq!(vector, (1.0, 2.0).into());

        //test dot
        let vec1 = Vec2::new(3.0, 4.0);
        let vec2 = Vec2::new(5.0, 6.0);
        let dot = vec1.dot(vec2);
        assert_eq!(dot, 39.0);

        //test angle
        let vec1 = Vec2::new(1.0, 0.0);
        let vec2 = Vec2::new(0.0, 1.0);
        let res = vec1.angle(vec2).to_degrees();
        assert!(f64::abs(res - 90.0) < 0.001);

        //test lerp
        let vec1 = Vec2::new(0.0, 0.0);
        let vec2 = Vec2::new(4.0, 4.0);
        assert_eq!(vec1.lerp(vec2, 0.0), vec1);
        assert_eq!(vec1.lerp(vec2, 0.25), Vec2::new(1.0, 1.0));
        assert_eq!(vec1.lerp(vec2, 0.50), Vec2::new(2.0, 2.0));
        assert_eq!(vec1.lerp(vec2, 0.75), Vec2::new(3.0, 3.0));
        assert_eq!(vec1.lerp(vec2, 1.0), vec2);

        //test nlerp
        let vec1 = Vec2::new(-200.0, 0.0);
        let vec2 = Vec2::new(0.0, 200.0);
        assert_eq!(vec1.nlerp(vec2, 0.0), vec1.normalize());
        assert_eq!(vec1.nlerp(vec2, 0.5), (-0.7071067811865475, 0.7071067811865475).into()
        );
        assert_eq!(vec1.nlerp(vec2, 1.0), vec2.normalize());

        //test slerp
        let vec1 = Vec2::new(1.0, 0.0);
        let vec2 = Vec2::new(0.0, 1.0);
        assert!(vec1.slerp(vec2, 0.0).approx_eq(vec1, f32::EPSILON));
        assert!(vec1.slerp(vec2, 0.25).approx_eq(Vec2::new(0.92387, 0.38268), 0.0001));
        assert!(vec1.slerp(vec2, 0.5).approx_eq(Vec2::new(0.70710, 0.70710), 0.0001));
        assert!(vec1.slerp(vec2, 0.75).approx_eq(Vec2::new(0.38268, 0.92387), 0.0001));
        assert!(vec1.slerp(vec2, 1.0).approx_eq(vec2, f32::EPSILON));

        //test negation
        let vector = Vec2::new(1.0, 2.0);
        let vector = -vector;
        assert_eq!(vector, (-1.0, -2.0).into());

        //test add op
        let vector = Vec2::new(1.0, 2.0);
        let vector = vector + vector;
        assert_eq!(vector, (2.0, 4.0).into());

        //test sub op
        let vector = Vec2::new(1.0, 2.0);
        let vector = vector - vector;
        assert_eq!(vector, (0.0, 0.0).into());

        //test mul op
        let vector = Vec2::new(1.0, 2.0);
        let vector = vector * 2.0;
        assert_eq!(vector, (2.0, 4.0).into());

        //test add assign op
        let mut vector = Vec2::new(1.0, 2.0);
        vector += vector;
        assert_eq!(vector, (2.0, 4.0).into());

        //test sub assign op
        let mut vector = Vec2::new(1.0, 2.0);
        vector -= vector;
        assert_eq!(vector, (0.0, 0.0).into());

        //test mul assign op
        let mut vector = Vec2::new(1.0, 2.0);
        vector *= 2.0;
        assert_eq!(vector, (2.0, 4.0).into());

        //test default
        let vector = Vec2::default();
        assert_eq!(vector, (0.0, 0.0).into());
    }

    #[test]
    fn test_vec_3() {
        //test new
        let vector = Vec3::new(2.0, 1.0, 3.0);
        assert_eq!(vector, (2.0, 1.0, 3.0).into());

        //test unit_x
        let vector = Vec3::unit_x();
        assert_eq!(vector, (1.0, 0.0, 0.0).into());

        //test unit_y
        let vector = Vec3::unit_y();
        assert_eq!(vector, (0.0, 1.0, 0.0).into());

        //test unit_z
        let vector = Vec3::unit_z();
        assert_eq!(vector, (0.0, 0.0, 1.0).into());

        //test zeroed
        assert_eq!(Vec3::zeroed(), (0.0, 0.0, 0.0).into());

        //test add
        let vector = Vec3::new(0.0, 0.0, 0.0);
        let vector = vector.add((5.0, 4.0, 3.0).into());
        assert_eq!(vector, (5.0, 4.0, 3.0).into());

        //test sub
        let vector = Vec3::new(-5.0, -4.0, -3.0);
        let vector = vector.sub((-10.0, -8.0, -6.0).into());
        assert_eq!(vector, (5.0, 4.0, 3.0).into());

        //test scale
        let vector = vector.scale(5.0);
        assert_eq!(vector, (25.0, 20.0, 15.0).into());

        //test magnitude
        let mag = vector.magnitude();
        assert!(f32::abs(mag - 35.35534) < f32::EPSILON);

        //test sq_magnitude
        let sq_mag = vector.sq_magnitude();
        assert_eq!(sq_mag, 1250.0);

        //test normalize
        let vector = vector.normalize();
        assert_eq!(vector, (0.70710677, 0.56568545, 0.42426407).into());

        //test distance
        let from = Vec3::zeroed();
        let to = Vec3::new(1.0, -2.0, 1.0);
        let vector = from.distance(to);
        assert_eq!(vector, (1.0, -2.0, 1.0).into());

        //test dot
        let vec1 = Vec3::new(3.0, 4.0, 5.0);
        let vec2 = Vec3::new(5.0, 6.0, 7.0);
        let dot = vec1.dot(vec2);
        assert_eq!(dot, 74.0);

        //test angle
        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(-3.0, 0.0, 0.0);
        let res = vec1.angle(vec2).to_degrees();
        assert!(f64::abs(res - 180.0) < 0.001);

        //test lerp
        let vec1 = Vec3::new(0.0, 0.0, 0.0);
        let vec2 = Vec3::new(4.0, 4.0, 4.0);
        assert_eq!(vec1.lerp(vec2, 0.0), vec1);
        assert_eq!(vec1.lerp(vec2, 0.25), Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(vec1.lerp(vec2, 0.50), Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(vec1.lerp(vec2, 0.75), Vec3::new(3.0, 3.0, 3.0));
        assert_eq!(vec1.lerp(vec2, 1.0), vec2);

        //test nlerp
        let vec1 = Vec3::new(-200.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 200.0, 0.0);
        assert_eq!(vec1.nlerp(vec2, 0.0), vec1.normalize());
        assert_eq!(
            vec1.nlerp(vec2, 0.5),
            (-0.7071067811865475, 0.7071067811865475, 0.0).into()
        );
        assert_eq!(vec1.nlerp(vec2, 1.0), vec2.normalize());

        //test slerp
        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 1.0, 0.0);
        assert!(vec1.slerp(vec2, 0.0).approx_eq(vec1, f32::EPSILON));
        assert!(vec1.slerp(vec2, 0.25).approx_eq(Vec3::new(0.92387, 0.38268, 0.0), 0.0001));
        assert!(vec1.slerp(vec2, 0.5).approx_eq(Vec3::new(0.70710, 0.70710, 0.0), 0.0001));
        assert!(vec1.slerp(vec2, 0.75).approx_eq(Vec3::new(0.38268, 0.92387, 0.0), 0.0001));

        //test negation
        let vector = -Vec3::new(5.0, 4.0, 3.0);
        assert_eq!(vector, (-5.0, -4.0, -3.0).into());

        //test add op
        let vector = Vec3::new(1.0, 2.0, 3.0);
        let vector = vector + vector;
        assert_eq!(vector, (2.0, 4.0, 6.0).into());

        //test sub op
        let vector = Vec3::new(1.0, 2.0, 3.0);
        let vector = vector - vector;
        assert_eq!(vector, (0.0, 0.0, 0.0).into());

        //test mul op
        let vector = Vec3::new(1.0, 2.0, 3.0);
        let vector = vector * 2.0;
        assert_eq!(vector, (2.0, 4.0, 6.0).into());

        //test add assign op
        let mut vector = Vec3::new(1.0, 2.0, 3.0);
        vector += vector;
        assert_eq!(vector, (2.0, 4.0, 6.0).into());

        //test sub assign op
        let mut vector = Vec3::new(1.0, 2.0, 3.0);
        vector -= vector;
        assert_eq!(vector, (0.0, 0.0, 0.0).into());

        //test mul assign op
        let mut vector = Vec3::new(1.0, 2.0, 3.0);
        vector *= 2.0;
        assert_eq!(vector, (2.0, 4.0, 6.0).into());

        //test default
        let vector = Vec3::default();
        assert_eq!(vector, (0.0, 0.0, 0.0).into());
    }

    #[test]
    fn test_vec_4() {
        //test new
        let vector = Vec4::new(2.0, 1.0, 3.0, 4.0);
        assert_eq!(vector, (2.0, 1.0, 3.0, 4.0).into());

        //test unit_x
        let vector = Vec4::unit_x();
        assert_eq!(vector, (1.0, 0.0, 0.0, 0.0).into());

        //test unit_y
        let vector = Vec4::unit_y();
        assert_eq!(vector, (0.0, 1.0, 0.0, 0.0).into());

        //test unit_z
        let vector = Vec4::unit_z();
        assert_eq!(vector, (0.0, 0.0, 1.0, 0.0).into());

        //test unit_w
        let vector = Vec4::unit_w();
        assert_eq!(vector, (0.0, 0.0, 0.0, 1.0).into());

        //test zeroed
        assert_eq!(Vec4::zeroed(), (0.0, 0.0, 0.0, 0.0).into());

        //test add
        let vector = Vec4::new(0.0, 0.0, 0.0, 0.0);
        let vector = vector.add((5.0, 4.0, 3.0, 2.0).into());
        assert_eq!(vector, (5.0, 4.0, 3.0, 2.0).into());

        //test sub
        let vector = Vec4::new(-5.0, -4.0, -3.0, -2.0);
        let vector = vector.sub((-10.0, -8.0, -6.0, -4.0).into());
        assert_eq!(vector, (5.0, 4.0, 3.0, 2.0).into());

        //test scale
        let vector = vector.scale(5.0);
        assert_eq!(vector, (25.0, 20.0, 15.0, 10.0).into());

        //test magnitude
        let mag = vector.magnitude();
        println!("{}", mag);
        assert!(f32::abs(mag - 36.742348) < f32::EPSILON);

        //test sq_magnitude
        let sq_mag = vector.sq_magnitude();
        assert_eq!(sq_mag, 1350.0);

        //test normalize
        let vector = vector.normalize();
        assert_eq!(vector, (0.6804138, 0.544331, 0.40824828, 0.2721655).into());

        //test distance
        let from = Vec4::zeroed();
        let to = Vec4::new(1.0, -2.0, 1.0, 4.4);
        let vector = from.distance(to);
        assert_eq!(vector, (1.0, -2.0, 1.0, 4.4).into());

        //test dot
        let vec1 = Vec4::new(3.0, 4.0, 5.0, 6.0);
        let vec2 = Vec4::new(5.0, 6.0, 7.0, 8.0);
        let dot = vec1.dot(vec2);
        assert_eq!(dot, 122.0);

        //test angle
        let vec1 = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let vec2 = Vec4::new(0.0, 4.0, 0.0, 0.0);
        let res = vec1.angle(vec2).to_degrees();
        assert!(f64::abs(res - 90.0) < 0.001);

        //test lerp
        let vec1 = Vec4::new(0.0, 0.0, 0.0, 0.0);
        let vec2 = Vec4::new(4.0, 4.0, 4.0, 4.0);
        assert_eq!(vec1.lerp(vec2, 0.0), vec1);
        assert_eq!(vec1.lerp(vec2, 0.25), Vec4::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(vec1.lerp(vec2, 0.50), Vec4::new(2.0, 2.0, 2.0, 2.0));
        assert_eq!(vec1.lerp(vec2, 0.75), Vec4::new(3.0, 3.0, 3.0, 3.0));
        assert_eq!(vec1.lerp(vec2, 1.0), vec2);

        //test slerp
        let vec1 = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let vec2 = Vec4::new(0.0, 1.0, 0.0, 0.0);
        assert!(vec1.slerp(vec2, 0.0).approx_eq(vec1, f32::EPSILON));
        assert!(vec1.slerp(vec2, 0.25).approx_eq(Vec4::new(0.92387, 0.38268, 0.0, 0.0), 0.0001));
        assert!(vec1.slerp(vec2, 0.5).approx_eq(Vec4::new(0.70710, 0.70710, 0.0, 0.0), 0.0001));
        assert!(vec1.slerp(vec2, 0.75).approx_eq(Vec4::new(0.38268, 0.92387, 0.0, 0.0), 0.0001));
        assert!(vec1.slerp(vec2, 1.0).approx_eq(vec2, f32::EPSILON));

        //test nlerp
        let vec1 = Vec4::new(-200.0, 0.0, 0.0, 0.0);
        let vec2 = Vec4::new(0.0, 200.0, 0.0, 0.0);
        assert_eq!(vec1.nlerp(vec2, 0.0), vec1.normalize());
        assert_eq!(vec1.nlerp(vec2, 0.5), (-0.7071067811865475, 0.7071067811865475, 0.0, 0.0).into());
        assert_eq!(vec1.nlerp(vec2, 1.0), vec2.normalize());

        //test negation
        let vector = Vec4::new(5.0, 4.0, 3.0, 2.0);
        let vector = -vector;
        assert_eq!(vector, (-5.0, -4.0, -3.0, -2.0).into());

        //test add op
        let vector = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let vector = vector + vector;
        assert_eq!(vector, (2.0, 4.0, 6.0, 8.0).into());

        //test sub op
        let vector = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let vector = vector - vector;
        assert_eq!(vector, (0.0, 0.0, 0.0, 0.0).into());

        //test mul op
        let vector = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let vector = vector * 2.0;
        assert_eq!(vector, (2.0, 4.0, 6.0, 8.0).into());

        //test add assign op
        let mut vector = Vec4::new(1.0, 2.0, 3.0, 4.0);
        vector += vector;
        assert_eq!(vector, (2.0, 4.0, 6.0, 8.0).into());

        //test sub assign op
        let mut vector = Vec4::new(1.0, 2.0, 3.0, 4.0);
        vector -= vector;
        assert_eq!(vector, (0.0, 0.0, 0.0, 0.0).into());

        //test mul assign op
        let mut vector = Vec4::new(1.0, 2.0, 3.0, 4.0);
        vector *= 2.0;
        assert_eq!(vector, (2.0, 4.0, 6.0, 8.0).into());

        //test default
        let vector = Vec4::default();
        assert_eq!(vector, (0.0, 0.0, 0.0, 0.0).into());
    }

    #[test]
    fn test_mat_2() {
        //test new
        let mat = Mat2::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(mat, [1.0, 2.0, 3.0, 4.0].into());

        //test from_cols
        let v1 = Vec2::new(1.0, 3.0);
        let v2 = Vec2::new(2.0, 4.0);
        let mat = Mat2::from_cols(v1, v2);
        assert_eq!(mat, [1.0, 2.0, 3.0, 4.0].into());

        //test zeroed
        let zero = Mat2::zeroed();
        assert_eq!(zero, [0.0, 0.0, 0.0, 0.0].into());

        //test identity
        let ident = Mat2::identity();
        assert_eq!(ident, [1.0, 0.0, 0.0, 1.0].into());

        //test get_element
        let mat = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let elem = mat.get_element(2);
        assert_eq!(elem, 3.0);

        //test get_column
        let mat = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let column = mat.get_column(matrix::Column::Y);
        assert_eq!(column, Vec2::new(2.0, 4.0));

        //test get_row
        let mat = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let row = mat.get_row(matrix::Row::X);
        assert_eq!(row, Vec2::new(1.0, 2.0));

        //test add
        let mat = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let res = mat + mat;
        assert_eq!(res, Mat2::new(2.0, 4.0, 6.0, 8.0));

        //test sub
        let mat = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let res = mat - mat;
        assert_eq!(res, Mat2::zeroed());

        //test scale
        let mat = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let res = mat * 2.0;
        assert_eq!(res, Mat2::new(2.0, 4.0, 6.0, 8.0));

        //test mult_mat
        let mat = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let res = mat * mat;
        assert_eq!(res, Mat2::new(7.0, 10.0, 15.0, 22.0));

        //test mult_vec
        let mat = Mat2::new(1.0, 2.0, 3.0, 4.0);
        let vec = Vec2::new(1.0, 2.0);
        let res = mat * vec;
        assert_eq!(res, Vec2::new(5.0, 11.0));
    }

    #[test]
    fn test_mat_3() {
        //test new
        let mat = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        assert_eq!(mat, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].into());

        //test from_cols
        let v1 = Vec3::new(1.0, 4.0, 7.0);
        let v2 = Vec3::new(2.0, 5.0, 8.0);
        let v3 = Vec3::new(3.0, 6.0, 9.0);
        let mat = Mat3::from_cols(v1, v2, v3);
        assert_eq!(mat, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].into());

        //test zeroed
        let zero = Mat3::zeroed();
        assert_eq!(zero, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0].into());

        //test identity
        let ident = Mat3::identity();
        assert_eq!(ident, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0].into());

        //test get_element
        let mat = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let elem = mat.get_element(7);
        assert_eq!(elem, 8.0);

        //test get_column
        let mat = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let column = mat.get_column(matrix::Column::Z);
        assert_eq!(column, Vec3::new(3.0, 6.0, 9.0));

        //test get_row
        let mat = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let row = mat.get_row(matrix::Row::Z);
        assert_eq!(row, Vec3::new(7.0, 8.0, 9.0));

        //test add
        let mat = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let res = mat + mat;
        assert_eq!(res, Mat3::new(2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0));

        //test sub
        let mat = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let res = mat - mat;
        assert_eq!(res, Mat3::zeroed());

        //test scale
        let mat = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let res = mat * 2.0;
        assert_eq!(res, Mat3::new(2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0));

        //test mult_mat
        let mat = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let res = mat * mat;
        assert_eq!(res, Mat3::new(30.0, 36.0, 42.0, 66.0, 81.0, 96.0, 102.0, 126.0, 150.0));

        //test mult_vec
        let mat = Mat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let res = mat * vec;
        assert_eq!(res, Vec3::new(14.0, 32.0, 50.0));
    }

    #[test]
    fn test_vec_2_conversions() {
        let v1 = [1.0, 2.0];
        let v2 = (1.0, 2.0);
        let v3 = Vec3::new(-1.1, 2.2, -3.3);
        let v4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        let test1: Vec2<_> = v1.into();
        let test2: Vec2<_> = v2.into();
        let test3: Vec2<_> = v3.into();
        let test4: Vec2<_> = v4.into();

        assert_eq!(test1, Vec2::new(1.0, 2.0));
        assert_eq!(test2, Vec2::new(1.0, 2.0));
        assert_eq!(test3, Vec2::new(-1.1, 2.2));
        assert_eq!(test4, Vec2::new(1.0, 2.0));
    }

    #[test]
    fn test_vec_3_conversions() {
        let v1 = [1.0, 2.0, 3.0];
        let v2 = (1.0, 2.0, 3.0);
        let v3 = Vec2::new(-1.1, 2.2);
        let v4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        let test1: Vec3<_> = v1.into();
        let test2: Vec3<_> = v2.into();
        let test3: Vec3<_> = v3.into();
        let test4: Vec3<_> = v4.into();

        assert_eq!(test1, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(test2, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(test3, Vec3::new(-1.1, 2.2, 0.0));
        assert_eq!(test4, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vec_4_conversions() {
        let v1 = [1.0, 2.0, 3.0, 4.0];
        let v2 = (1.0, 2.0, 3.0, 4.0);
        let v3 = Vec2::new(-1.1, 2.2);
        let v4 = Vec3::new(1.0, 2.0, 3.0);

        let test1: Vec4<_> = v1.into();
        let test2: Vec4<_> = v2.into();
        let test3: Vec4<_> = v3.into();
        let test4: Vec4<_> = v4.into();

        assert_eq!(test1, Vec4::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(test2, Vec4::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(test3, Vec4::new(-1.1, 2.2, 0.0, 0.0));
        assert_eq!(test4, Vec4::new(1.0, 2.0, 3.0, 0.0));
    }

    #[test]
    fn test_mat_2_conversions() {
        let m2 = Mat2 {
            m00: 2.0,
            m01: 3.0,
            m10: 4.0,
            m11: 5.0,
            // x: (2.0, 3.0).into(),
            // y: (4.0, 5.0).into(),
        };

        assert_eq!(m2, [[2.0, 3.0], [4.0, 5.0]].into());
        assert_eq!(m2, [2.0, 3.0, 4.0, 5.0].into());
    }

    #[test]
    fn test_mat_3_conversions() {
        let m3 = Mat3 {
            m00: 1.0,
            m01: 2.0,
            m02: 3.0,
            m10: 4.0,
            m11: 5.0,
            m12: 6.0,
            m20: 7.0,
            m21: 8.0,
            m22: 9.0,

            // x: (1.0, 2.0, 3.0).into(),
            // y: (4.0, 5.0, 6.0).into(),
            // z: (7.0, 8.0, 9.0).into(),
        };

        assert_eq!(m3,[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]].into());
        assert_eq!(m3, [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0].into());
    }

    #[test]
    fn test_mat_4_conversions() {
        let m4 = Mat4 {
            x: (1.0, 2.0, 3.0, 4.0).into(),
            y: (1.0, 2.0, 3.0, 4.0).into(),
            z: (1.0, 2.0, 3.0, 4.0).into(),
            w: (1.0, 2.0, 3.0, 4.0).into(),
        };

        assert_eq!(m4, [[1.0, 2.0, 3.0, 4.0], [1.0, 2.0, 3.0, 4.0], [1.0, 2.0, 3.0, 4.0], [1.0, 2.0, 3.0, 4.0]].into());
        assert_eq!(m4, [1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0].into());
    }
}
