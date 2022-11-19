#![allow(non_snake_case)]

use super::base::Float;
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign},
};

///////////////////////////////////////////////////////////////////////////////////////////////////
// Traits
///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait VectorOps {
    type Float;

    fn zeroed() -> Self;
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn scale(self, other: Self::Float) -> Self;
    fn magnitude(self) -> Self::Float;
    fn sq_magnitude(self) -> Self::Float;
    fn normalize(self) -> Self;
    fn distance(self, other: Self) -> Self;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Statics
///////////////////////////////////////////////////////////////////////////////////////////////////

pub mod Vector {
    use super::VectorOps;
    pub fn add<T: VectorOps>(vector1: T, vector2: T) -> T {
        vector1.add(vector2)
    }
    pub fn sub<T: VectorOps>(vector1: T, vector2: T) -> T {
        vector1.sub(vector2)
    }
    pub fn scale<T: VectorOps>(vector: T, scalar: <T as VectorOps>::Float) -> T {
        vector.scale(scalar)
    }
    pub fn magnitude<T: VectorOps>(vector: T) -> <T as VectorOps>::Float {
        vector.magnitude()
    }
    pub fn sq_magnitude<T: VectorOps>(vector: T) -> <T as VectorOps>::Float {
        vector.sq_magnitude()
    }
    pub fn normalize<T: VectorOps>(vector: T) -> T {
        vector.normalize()
    }
    pub fn distance<T: VectorOps>(from: T, to: T) -> T {
        from.distance(to)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Vec2
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Vec2<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Float,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn unit_x() -> Self {
        Self {
            x: T::one(),
            y: T::zero(),
        }
    }

    pub fn unit_y() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
        }
    }
}

impl<T> VectorOps for Vec2<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
    T: Copy,
{
    type Float = T;

    fn zeroed() -> Self {
        Vec2::new(T::zero(), T::zero())
    }

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn sub(self, other: Self) -> Self {
        self - other
    }

    fn scale(self, other: T) -> Self {
        self * other
    }

    fn magnitude(self) -> Self::Float {
        Float::sqrt((self.x * self.x) + (self.y * self.y))
    }

    fn sq_magnitude(self) -> Self::Float {
        (self.x * self.x) + (self.y * self.y)
    }

    fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    fn distance(self, other: Self) -> Self {
        other - self
    }
}

impl<T> Neg for Vec2<T>
where
    T: Neg<Output = T>,
    T: Float,
{
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Add for Vec2<T>
where
    T: Float,
    T: Add<Output = T>,
{
    type Output = Vec2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vec2<T>
where
    T: Float,
    T: Sub<Output = T>,
{
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Copy,
{
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> AddAssign for Vec2<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T> SubAssign for Vec2<T>
where
    T: Float,
    T: Sub<Output = T>,
    T: Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T> MulAssign<T> for Vec2<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T> Display for Vec2<T>
where
    T: Float,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vec2({}, {})", self.x, self.y)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Vec3
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Vec3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Float,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn unit_x() -> Self {
        Self {
            x: T::one(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn unit_y() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
            z: T::zero(),
        }
    }

    pub fn unit_z() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::one(),
        }
    }
}

impl<T> VectorOps for Vec3<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
    T: Copy,
{
    type Float = T;

    fn zeroed() -> Self {
        Vec3::new(T::zero(), T::zero(), T::zero())
    }

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn sub(self, other: Self) -> Self {
        self - other
    }

    fn scale(self, other: Self::Float) -> Self {
        self * other
    }

    fn magnitude(self) -> Self::Float {
        Float::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z))
    }

    fn sq_magnitude(self) -> Self::Float {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    fn distance(self, other: Self) -> Self {
        other - self
    }
}

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T>,
    T: Float,
{
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Add for Vec3<T>
where
    T: Float,
    T: Add<Output = T>,
{
    type Output = Vec3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Float,
    T: Sub<Output = T>,
{
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Copy,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T> SubAssign for Vec3<T>
where
    T: Float,
    T: Sub<Output = T>,
    T: Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T> MulAssign<T> for Vec3<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T> Display for Vec3<T>
where
    T: Float,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Vec4
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Vec4<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec4<T>
where
    T: Float,
{
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn unit_x() -> Self {
        Self {
            x: T::one(),
            y: T::zero(),
            z: T::zero(),
            w: T::zero(),
        }
    }

    pub fn unit_y() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
            z: T::zero(),
            w: T::zero(),
        }
    }

    pub fn unit_z() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::one(),
            w: T::zero(),
        }
    }

    pub fn unit_w() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
            w: T::one(),
        }
    }
}

impl<T> VectorOps for Vec4<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
    T: Copy,
{
    type Float = T;

    fn zeroed() -> Self {
        Vec4::new(T::zero(), T::zero(), T::zero(), T::zero())
    }

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn sub(self, other: Self) -> Self {
        self - other
    }

    fn scale(self, other: Self::Float) -> Self {
        self * other
    }

    fn magnitude(self) -> Self::Float {
        Float::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w))
    }

    fn sq_magnitude(self) -> Self::Float {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)
    }

    fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    fn distance(self, other: Self) -> Self {
        other - self
    }
}

impl<T> Neg for Vec4<T>
where
    T: Neg<Output = T>,
    T: Float,
{
    type Output = Vec4<T>;

    fn neg(self) -> Self::Output {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl<T> Add for Vec4<T>
where
    T: Float,
    T: Add<Output = T>,
{
    type Output = Vec4<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T> Sub for Vec4<T>
where
    T: Float,
    T: Sub<Output = T>,
{
    type Output = Vec4<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<T> Mul<T> for Vec4<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Copy,
{
    type Output = Vec4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<T> AddAssign for Vec4<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<T> SubAssign for Vec4<T>
where
    T: Float,
    T: Sub<Output = T>,
    T: Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T> MulAssign<T> for Vec4<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T> Display for Vec4<T>
where
    T: Float,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vec4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Vec2 Conversions
///////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> From<[T; 2]> for Vec2<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [T; 2]) -> Self {
        Vec2 {
            x: value[0],
            y: value[1],
        }
    }
}

impl<T> From<(T, T)> for Vec2<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: (T, T)) -> Self {
        Vec2 {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<Vec3<T>> for Vec2<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Vec3<T>) -> Self {
        Vec2 {
            x: value.x,
            y: value.y,
        }
    }
}

impl<T> From<Vec4<T>> for Vec2<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Vec4<T>) -> Self {
        Vec2 {
            x: value.x,
            y: value.y,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Vec3 Conversions
///////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> From<[T; 3]> for Vec3<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [T; 3]) -> Self {
        Vec3 {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl<T> From<(T, T, T)> for Vec3<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: (T, T, T)) -> Self {
        Vec3 {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl<T> From<Vec2<T>> for Vec3<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Vec2<T>) -> Self {
        Vec3 {
            x: value.x,
            y: value.y,
            z: T::default(),
        }
    }
}

impl<T> From<Vec4<T>> for Vec3<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Vec4<T>) -> Self {
        Vec3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Vec4 Conversions
///////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> From<[T; 4]> for Vec4<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [T; 4]) -> Self {
        Vec4 {
            x: value[0],
            y: value[1],
            z: value[2],
            w: value[3],
        }
    }
}

impl<T> From<(T, T, T, T)> for Vec4<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: (T, T, T, T)) -> Self {
        Vec4 {
            x: value.0,
            y: value.1,
            z: value.2,
            w: value.3,
        }
    }
}

impl<T> From<Vec2<T>> for Vec4<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Vec2<T>) -> Self {
        Vec4 {
            x: value.x,
            y: value.y,
            z: T::default(),
            w: T::default(),
        }
    }
}

impl<T> From<Vec3<T>> for Vec4<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Vec3<T>) -> Self {
        Vec4 {
            x: value.x,
            y: value.y,
            z: value.z,
            w: T::default(),
        }
    }
}
