use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

use crate::VectorOps;

use super::base::Float;
use super::vector::Vec3;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Quaternion<T: Float> {
    pub scalar: T,
    pub vector: Vec3<T>,
}

impl<T> Quaternion<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
{
    pub fn new(scalar: T, vector: Vec3<T>) -> Self {
        Self { scalar, vector }
    }

    pub fn product(&self, q: &Quaternion<T>) -> Self {
        let scalar = self.scalar * q.scalar - self.vector.dot(q.vector);
        let imaginary =
            q.vector * self.scalar + self.vector * q.scalar + self.vector.cross(q.vector);

        Self {
            scalar,
            vector: imaginary,
        }
    }

    pub fn magnitude(&self) -> T {
        let scalar = self.scalar * self.scalar;
        let imaginary = self.vector.dot(self.vector);

        Float::sqrt(scalar + imaginary)
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag != T::zero() {
            let mag_value = T::one() / mag;

            self.scalar *= mag_value;
            self.vector *= mag_value;
        }
    }

    pub fn unit_norm(&mut self) {
        let angle = T::to_radians(self.scalar);
        self.vector = self.vector.normalize();

        self.scalar = T::cos(angle * T::one_half());
        self.vector *= T::sin(angle * T::one_half());
    }

    pub fn conjugate(&self) -> Self {
        let scalar = self.scalar;
        let imaginary = self.vector * T::neg_one();

        Self {
            scalar,
            vector: imaginary,
        }
    }

    pub fn inverse(&self) -> Self {
        let mut absolute_value = self.magnitude();
        absolute_value *= absolute_value;
        absolute_value = T::one() / absolute_value;

        let conjugate_value = self.conjugate();

        let scalar = conjugate_value.scalar * absolute_value;
        let imaginary = conjugate_value.vector * absolute_value;

        Self {
            scalar,
            vector: imaginary,
        }
    }
}

pub fn rotate_on_axis<T>(starting_vector: Vec3<T>, angle: T, axis: Vec3<T>) -> Vec3<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
{
    let pure_quaternion = Quaternion::new(T::zero(), starting_vector);

    let axis = axis.normalize();

    let mut real_quaternion = Quaternion::new(angle, axis);

    real_quaternion.unit_norm();

    let inverse = real_quaternion.inverse();

    let rotated_vector = real_quaternion * pure_quaternion * inverse;

    rotated_vector.vector
}

impl<T> Add for Quaternion<T>
where
    T: Float,
    T: Add<Output = T>,
{
    type Output = Quaternion<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            scalar: self.scalar + rhs.scalar,
            vector: self.vector + rhs.vector,
        }
    }
}

impl<T> AddAssign for Quaternion<T>
where
    T: Float,
    T: Add<Output = T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.scalar = self.scalar + rhs.scalar;
        self.vector = self.vector + rhs.vector;
    }
}

impl<T> Sub for Quaternion<T>
where
    T: Float,
    T: Sub<Output = T>,
{
    type Output = Quaternion<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            scalar: self.scalar - rhs.scalar,
            vector: self.vector - rhs.vector,
        }
    }
}

impl<T> SubAssign for Quaternion<T>
where
    T: Float,
    T: Sub<Output = T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.scalar = self.scalar - rhs.scalar;
        self.vector = self.vector - rhs.vector;
    }
}

impl<T> Mul for Quaternion<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Div<Output = T>,
    T: Mul<Output = T>,
    T: Copy,
{
    type Output = Quaternion<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let scalar = self.scalar * rhs.scalar - self.vector.dot(rhs.vector);
        let imaginary =
            rhs.vector * self.scalar + self.vector * rhs.scalar + self.vector.cross(rhs.vector);

        Self {
            scalar,
            vector: imaginary,
        }
    }
}

impl<T> Mul<T> for Quaternion<T>
where
    T: Float,

    T: Mul<Output = T>,
{
    type Output = Quaternion<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let scalar = self.scalar * rhs;
        let imaginary = self.vector * rhs;

        Self {
            scalar,
            vector: imaginary,
        }
    }
}

impl<T> MulAssign for Quaternion<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Div<Output = T>,
    T: Mul<Output = T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
