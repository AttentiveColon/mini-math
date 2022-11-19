#![allow(dead_code)]

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

///////////////////////////////////////////////////////////////////////////////////////////////////
// Traits
///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Float {
    fn sqrt(self) -> Self;
    fn one() -> Self;
    fn zero() -> Self;
}

impl Float for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }

    fn one() -> Self {
        1.0f32
    }

    fn zero() -> Self {
        0.0f32
    }
}

impl Float for f64 {
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }

    fn one() -> Self {
        1.0f64
    }

    fn zero() -> Self {
        0.0f64
    }
}

pub trait VectorOps {
    type Float;

    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn scale(self, other: Self::Float) -> Self;
    fn magnitude(self) -> Self::Float;
    fn normalize(self) -> Self;
}

pub trait MatrixOps {
    type Float;
    type Vector;

    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn scale(self, other: Self::Float) -> Self;
    fn mult_mat(self, other: Self) -> Self;
    fn mult_vec(self, other: Self::Vector) -> Self::Vector;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Statics
///////////////////////////////////////////////////////////////////////////////////////////////////

pub mod vector {
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
    pub fn normalize<T: VectorOps>(vector: T) -> T {
        vector.normalize()
    }
}

pub mod matrix {
    use super::MatrixOps;
    pub fn add<T: MatrixOps>(matrix1: T, matrix2: T) -> T {
        matrix1.add(matrix2)
    }
    pub fn sub<T: MatrixOps>(matrix1: T, matrix2: T) -> T {
        matrix1.sub(matrix2)
    }
    pub fn scale<T: MatrixOps>(matrix1: T, scalar: T::Float) -> T {
        matrix1.scale(scalar)
    }
    pub fn mult_mat<T: MatrixOps>(matrix1: T, matrix2: T) -> T {
        matrix1.mult_mat(matrix2)
    }
    pub fn mult_vec<T: MatrixOps>(matrix: T, vector: T::Vector) -> T::Vector {
        matrix.mult_vec(vector)
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

    fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
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

    fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
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

    fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
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

///////////////////////////////////////////////////////////////////////////////////////////////////
// Mat2
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Mat2<T: Float> {
    pub x: Vec2<T>,
    pub y: Vec2<T>,
}

impl<T> Mat2<T>
where
    T: Float,
    T: Default,
{
    pub fn new(x: Vec2<T>, y: Vec2<T>) -> Self {
        Mat2 { x, y }
    }
}

impl<T> MatrixOps for Mat2<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Copy,
{
    type Float = T;
    type Vector = Vec2<T>;

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn sub(self, other: Self) -> Self {
        self - other
    }

    fn scale(self, other: T) -> Self {
        self * other
    }

    fn mult_mat(self, other: Self) -> Self {
        self * other
    }

    fn mult_vec(self, other: Self::Vector) -> Self::Vector {
        self * other
    }
}

impl<T> Add for Mat2<T>
where
    T: Float,
    T: Add<Output = T>,
{
    type Output = Mat2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Mat2<T>
where
    T: Float,
    T: Sub<Output = T>,
{
    type Output = Mat2<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Mat2<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Copy,
{
    type Output = Mat2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Mul for Mat2<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Copy,
{
    type Output = Mat2<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: Vec2::new(
                self.x.x * rhs.x.x + self.x.y * rhs.y.x,
                self.x.x * rhs.x.y + self.x.y * rhs.y.y,
            ),
            y: Vec2::new(
                self.y.x * rhs.x.x + self.y.y * rhs.y.x,
                self.y.x * rhs.x.y + self.y.y * rhs.y.y,
            ),
        }
    }
}

impl<T> Mul<Vec2<T>> for Mat2<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Copy,
{
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Self::Output {
            x: self.x.x * rhs.x + self.x.y * rhs.y,
            y: self.y.x * rhs.x + self.y.y * rhs.y,
        }
    }
}

impl<T> From<[[T; 2]; 2]> for Mat2<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [[T; 2]; 2]) -> Self {
        Mat2 {
            x: (value[0][0], value[0][1]).into(),
            y: (value[1][0], value[1][1]).into(),
        }
    }
}

impl<T> From<[T; 4]> for Mat2<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [T; 4]) -> Self {
        Mat2 {
            x: (value[0], value[1]).into(),
            y: (value[2], value[3]).into(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Mat3
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Mat3<T: Float> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}

impl<T> Mat3<T>
where
    T: Float,
    T: Default,
{
    pub fn new(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Self {
        Mat3 { x, y, z }
    }
}

impl<T> MatrixOps for Mat3<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Copy,
{
    type Float = T;
    type Vector = Vec3<T>;

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn sub(self, other: Self) -> Self {
        self - other
    }

    fn scale(self, other: T) -> Self {
        self * other
    }

    fn mult_mat(self, other: Self) -> Self {
        self * other
    }

    fn mult_vec(self, other: Self::Vector) -> Self::Vector {
        self * other
    }
}

impl<T> Add for Mat3<T>
where
    T: Float,
    T: Add<Output = T>,
{
    type Output = Mat3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Mat3<T>
where
    T: Float,
    T: Sub<Output = T>,
{
    type Output = Mat3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul<T> for Mat3<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Copy,
{
    type Output = Mat3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Mul for Mat3<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Copy,
{
    type Output = Mat3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: Vec3::new(
                self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x,
                self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y,
                self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z,
            ),
            y: Vec3::new(
                self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x,
                self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y,
                self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z,
            ),
            z: Vec3::new(
                self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x,
                self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y,
                self.z.x * rhs.x.z + self.z.y * rhs.y.z + self.z.z * rhs.z.z,
            ),
        }
    }
}

impl<T> Mul<Vec3<T>> for Mat3<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Copy,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Self::Output {
            x: self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z,
            y: self.y.x * rhs.x + self.y.y * rhs.y + self.y.z * rhs.z,
            z: self.z.x * rhs.x + self.z.y * rhs.y + self.z.z * rhs.z,
        }
    }
}

impl<T> From<[[T; 3]; 3]> for Mat3<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [[T; 3]; 3]) -> Self {
        Mat3 {
            x: (value[0][0], value[0][1], value[0][2]).into(),
            y: (value[1][0], value[1][1], value[1][2]).into(),
            z: (value[2][0], value[2][1], value[2][2]).into(),
        }
    }
}

impl<T> From<[T; 9]> for Mat3<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [T; 9]) -> Self {
        Mat3 {
            x: (value[0], value[1], value[2]).into(),
            y: (value[3], value[4], value[5]).into(),
            z: (value[6], value[7], value[8]).into(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Mat4
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Mat4<T: Float> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

impl<T> Mat4<T>
where
    T: Float,
    T: Default,
{
    pub fn new(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Self {
        Mat4 { x, y, z, w }
    }
}

impl<T> MatrixOps for Mat4<T>
where
    T: Float,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Copy,
{
    type Float = T;
    type Vector = Vec4<T>;

    fn add(self, other: Self) -> Self {
        self + other
    }

    fn sub(self, other: Self) -> Self {
        self - other
    }

    fn scale(self, other: T) -> Self {
        self * other
    }

    fn mult_mat(self, other: Self) -> Self {
        self * other
    }

    fn mult_vec(self, other: Vec4<T>) -> Self::Vector {
        self * other
    }
}

impl<T> Add for Mat4<T>
where
    T: Float,
    T: Add<Output = T>,
{
    type Output = Mat4<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T> Sub for Mat4<T>
where
    T: Float,
    T: Sub<Output = T>,
{
    type Output = Mat4<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<T> Mul<T> for Mat4<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Copy,
{
    type Output = Mat4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<T> Mul for Mat4<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Copy,
{
    type Output = Mat4<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: Vec4::new(
                self.x.x * rhs.x.x + self.x.y * rhs.y.x + self.x.z * rhs.z.x + self.x.w * rhs.w.x,
                self.x.x * rhs.x.y + self.x.y * rhs.y.y + self.x.z * rhs.z.y + self.x.w * rhs.w.y,
                self.x.x * rhs.x.z + self.x.y * rhs.y.z + self.x.z * rhs.z.z + self.x.w * rhs.w.z,
                self.x.x * rhs.x.w + self.x.y * rhs.y.w + self.x.z * rhs.z.w + self.x.w * rhs.w.w,
            ),
            y: Vec4::new(
                self.y.x * rhs.x.x + self.y.y * rhs.y.x + self.y.z * rhs.z.x + self.y.w * rhs.w.x,
                self.y.x * rhs.x.y + self.y.y * rhs.y.y + self.y.z * rhs.z.y + self.y.w * rhs.w.y,
                self.y.x * rhs.x.z + self.y.y * rhs.y.z + self.y.z * rhs.z.z + self.y.w * rhs.w.z,
                self.y.x * rhs.x.w + self.y.y * rhs.y.w + self.y.z * rhs.z.w + self.y.w * rhs.w.w,
            ),
            z: Vec4::new(
                self.z.x * rhs.x.x + self.z.y * rhs.y.x + self.z.z * rhs.z.x + self.z.w * rhs.w.x,
                self.z.x * rhs.x.y + self.z.y * rhs.y.y + self.z.z * rhs.z.y + self.z.w * rhs.w.y,
                self.z.x * rhs.x.z + self.z.y * rhs.y.z + self.z.z * rhs.z.z + self.z.w * rhs.w.z,
                self.z.x * rhs.x.w + self.z.y * rhs.y.w + self.z.z * rhs.z.w + self.z.w * rhs.w.w,
            ),
            w: Vec4::new(
                self.w.x * rhs.x.x + self.w.y * rhs.y.x + self.w.z * rhs.z.x + self.w.w * rhs.w.x,
                self.w.x * rhs.x.y + self.w.y * rhs.y.y + self.w.z * rhs.z.y + self.w.w * rhs.w.y,
                self.w.x * rhs.x.z + self.w.y * rhs.y.z + self.w.z * rhs.z.z + self.w.w * rhs.w.z,
                self.w.x * rhs.x.w + self.w.y * rhs.y.w + self.w.z * rhs.z.w + self.w.w * rhs.w.w,
            ),
        }
    }
}

impl<T> Mul<Vec4<T>> for Mat4<T>
where
    T: Float,
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Copy,
{
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        Self::Output {
            x: self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z + self.x.w * rhs.w,
            y: self.y.x * rhs.x + self.y.y * rhs.y + self.y.z * rhs.z + self.y.w * rhs.w,
            z: self.z.x * rhs.x + self.z.y * rhs.y + self.z.z * rhs.z + self.z.w * rhs.w,
            w: self.w.x * rhs.x + self.w.y * rhs.y + self.w.z * rhs.z + self.w.w * rhs.w,
        }
    }
}

impl<T> From<[[T; 4]; 4]> for Mat4<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [[T; 4]; 4]) -> Self {
        Mat4 {
            x: (value[0][0], value[0][1], value[0][2], value[0][3]).into(),
            y: (value[1][0], value[1][1], value[1][2], value[1][3]).into(),
            z: (value[2][0], value[2][1], value[2][2], value[2][3]).into(),
            w: (value[3][0], value[3][1], value[3][2], value[3][3]).into(),
        }
    }
}

impl<T> From<[T; 16]> for Mat4<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [T; 16]) -> Self {
        Mat4 {
            x: (value[0], value[1], value[2], value[3]).into(),
            y: (value[4], value[5], value[6], value[7]).into(),
            z: (value[8], value[9], value[10], value[11]).into(),
            w: (value[12], value[13], value[14], value[15]).into(),
        }
    }
}
