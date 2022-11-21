use super::base::Float;
use super::vector::{Vec2, Vec3, Vec4};
use std::ops::{Add, Mul, Sub};

///////////////////////////////////////////////////////////////////////////////////////////////////
// Traits
///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait MatrixOps {
    type Float;
    type Vector;

    fn zeroed() -> Self;
    fn identity() -> Self;
    fn get_element(&self, index: usize) -> Self::Float;
    fn get_column(&self, col: Column) -> Self::Vector;
    fn get_row(&self, row: Row) -> Self::Vector;
    fn get_position(&self) -> Self::Vector;

    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn scale(self, other: Self::Float) -> Self;
    fn mult_mat(self, other: Self) -> Self;
    fn mult_vec(self, other: Self::Vector) -> Self::Vector;
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Enums
///////////////////////////////////////////////////////////////////////////////////////////////////

pub enum Column {
    X,
    Y,
    Z,
    W,
}

pub enum Row {
    X,
    Y,
    Z,
    W,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Statics
///////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(non_snake_case)]
pub mod Matrix {
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
// Mat2
///////////////////////////////////////////////////////////////////////////////////////////////////

#[rustfmt::skip]
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Mat2<T: Float> {
    pub m00: T, pub m01: T,
    pub m10: T, pub m11: T,
}

impl<T> Mat2<T>
where
    T: Float,
    T: Default,
    T: Copy,
{
    pub fn new(m00: T, m01: T, m10: T, m11: T) -> Self {
        Self { m00, m01, m10, m11 }
    }

    pub fn from_cols(col1: Vec2<T>, col2: Vec2<T>) -> Self {
        Self {
            m00: col1.x,
            m01: col2.x,
            m10: col1.y,
            m11: col2.y,
        }
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

    fn zeroed() -> Self {
        [T::zero(), T::zero(), T::zero(), T::zero()].into()
    }

    fn identity() -> Self {
        [T::one(), T::zero(), T::zero(), T::one()].into()
    }

    fn get_element(&self, index: usize) -> Self::Float {
        self.as_ref()[index]
    }

    fn get_column(&self, col: Column) -> Self::Vector {
        match col {
            Column::X => Vec2::new(self.m00, self.m10),
            Column::Y => Vec2::new(self.m01, self.m11),
            _ => panic!("outside bounds of mat2"),
        }
    }

    fn get_row(&self, row: Row) -> Self::Vector {
        match row {
            Row::X => Vec2::new(self.m00, self.m01),
            Row::Y => Vec2::new(self.m10, self.m11),
            _ => panic!("outside bounds of mat2"),
        }
    }

    fn get_position(&self) -> Self::Vector {
        unimplemented!()
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
            m00: self.m00 + rhs.m00,
            m01: self.m01 + rhs.m01,
            m10: self.m10 + rhs.m10,
            m11: self.m11 + rhs.m11,
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
            m00: self.m00 - rhs.m00,
            m01: self.m01 - rhs.m01,
            m10: self.m10 - rhs.m10,
            m11: self.m11 - rhs.m11,
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
            m00: self.m00 * rhs,
            m01: self.m01 * rhs,
            m10: self.m10 * rhs,
            m11: self.m11 * rhs,
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
            m00: self.m00 * rhs.m00 + self.m01 * rhs.m10,
            m01: self.m00 * rhs.m01 + self.m01 * rhs.m11,
            m10: self.m10 * rhs.m00 + self.m11 * rhs.m10,
            m11: self.m10 * rhs.m01 + self.m11 * rhs.m11,
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
            x: self.m00 * rhs.x + self.m01 * rhs.y,
            y: self.m10 * rhs.x + self.m11 * rhs.y,
        }
    }
}

impl<T> AsRef<[T; 4]> for Mat2<T>
where
    T: Float,
{
    fn as_ref(&self) -> &[T; 4] {
        unsafe { std::mem::transmute(self) }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Mat3
///////////////////////////////////////////////////////////////////////////////////////////////////

#[rustfmt::skip]
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Mat3<T: Float> {
    pub m00: T, pub m01: T, pub m02: T,
    pub m10: T, pub m11: T, pub m12: T,
    pub m20: T, pub m21: T, pub m22: T,
}

impl<T> Mat3<T>
where
    T: Float,
    T: Default,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(m00: T, m01: T, m02: T, m10: T, m11: T, m12: T, m20: T, m21: T, m22: T) -> Self {
        Self {
            m00,
            m01,
            m02,
            m10,
            m11,
            m12,
            m20,
            m21,
            m22,
        }
    }

    pub fn from_cols(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Self {
        Self {
            m00: x.x,
            m01: y.x,
            m02: z.x,
            m10: x.y,
            m11: y.y,
            m12: z.y,
            m20: x.z,
            m21: y.z,
            m22: z.z,
        }
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

    fn zeroed() -> Self {
        [
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
        ]
        .into()
    }

    fn identity() -> Self {
        [
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        ]
        .into()
    }

    fn get_element(&self, index: usize) -> Self::Float {
        self.as_ref()[index]
    }

    fn get_column(&self, col: Column) -> Self::Vector {
        match col {
            Column::X => Vec3::new(self.m00, self.m10, self.m20),
            Column::Y => Vec3::new(self.m01, self.m11, self.m21),
            Column::Z => Vec3::new(self.m02, self.m12, self.m22),
            _ => panic!("outside bounds of mat3"),
        }
    }

    fn get_row(&self, row: Row) -> Self::Vector {
        match row {
            Row::X => Vec3::new(self.m00, self.m01, self.m02),
            Row::Y => Vec3::new(self.m10, self.m11, self.m12),
            Row::Z => Vec3::new(self.m20, self.m21, self.m22),
            _ => panic!("outside bounds of mat3"),
        }
    }

    fn get_position(&self) -> Self::Vector {
        unimplemented!()
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
            m00: self.m00 + rhs.m00,
            m01: self.m01 + rhs.m01,
            m02: self.m02 + rhs.m02,
            m10: self.m10 + rhs.m10,
            m11: self.m11 + rhs.m11,
            m12: self.m12 + rhs.m12,
            m20: self.m20 + rhs.m20,
            m21: self.m21 + rhs.m21,
            m22: self.m22 + rhs.m22,
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
            m00: self.m00 - rhs.m00,
            m01: self.m01 - rhs.m01,
            m02: self.m02 - rhs.m02,
            m10: self.m10 - rhs.m10,
            m11: self.m11 - rhs.m11,
            m12: self.m12 - rhs.m12,
            m20: self.m20 - rhs.m20,
            m21: self.m21 - rhs.m21,
            m22: self.m22 - rhs.m22,
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
            m00: self.m00 * rhs,
            m01: self.m01 * rhs,
            m02: self.m02 * rhs,
            m10: self.m10 * rhs,
            m11: self.m11 * rhs,
            m12: self.m12 * rhs,
            m20: self.m20 * rhs,
            m21: self.m21 * rhs,
            m22: self.m22 * rhs,
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
            m00: self.m00 * rhs.m00 + self.m01 * rhs.m10 + self.m02 * rhs.m20,
            m01: self.m00 * rhs.m01 + self.m01 * rhs.m11 + self.m02 * rhs.m21,
            m02: self.m00 * rhs.m02 + self.m01 * rhs.m12 + self.m02 * rhs.m22,

            m10: self.m10 * rhs.m00 + self.m11 * rhs.m10 + self.m12 * rhs.m20,
            m11: self.m10 * rhs.m01 + self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m10 * rhs.m02 + self.m11 * rhs.m12 + self.m12 * rhs.m22,

            m20: self.m20 * rhs.m00 + self.m21 * rhs.m10 + self.m22 * rhs.m20,
            m21: self.m20 * rhs.m01 + self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m20 * rhs.m02 + self.m21 * rhs.m12 + self.m22 * rhs.m22,
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
            x: self.m00 * rhs.x + self.m01 * rhs.y + self.m02 * rhs.z,
            y: self.m10 * rhs.x + self.m11 * rhs.y + self.m12 * rhs.z,
            z: self.m20 * rhs.x + self.m21 * rhs.y + self.m22 * rhs.z,
        }
    }
}

impl<T> AsRef<[T; 9]> for Mat3<T>
where
    T: Float,
{
    fn as_ref(&self) -> &[T; 9] {
        unsafe { std::mem::transmute(self) }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Mat4
///////////////////////////////////////////////////////////////////////////////////////////////////

#[rustfmt::skip]
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Mat4<T: Float> {
    pub m00: T, pub m01: T, pub m02: T, pub m03: T,
    pub m10: T, pub m11: T, pub m12: T, pub m13: T,
    pub m20: T, pub m21: T, pub m22: T, pub m23: T,
    pub m30: T, pub m31: T, pub m32: T, pub m33: T,
}

impl<T> Mat4<T>
where
    T: Float,
    T: Default,
{
    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        m00: T, m01: T, m02: T, m03: T,
        m10: T, m11: T, m12: T, m13: T,
        m20: T, m21: T, m22: T, m23: T,
        m30: T, m31: T, m32: T, m33: T,
    ) -> Self {
        Mat4 {
            m00, m01, m02, m03,
            m10, m11, m12, m13,
            m20, m21, m22, m23,
            m30, m31, m32, m33,
        }
    }

    pub fn from_cols(x: Vec4<T>, y: Vec4<T>, z: Vec4<T>, w: Vec4<T>) -> Self {
        Mat4 {
            m00: x.x,
            m01: y.x,
            m02: z.x,
            m03: w.x,
            m10: x.y,
            m11: y.y,
            m12: z.y,
            m13: w.y,
            m20: x.z,
            m21: y.z,
            m22: z.z,
            m23: w.z,
            m30: x.w,
            m31: y.w,
            m32: z.w,
            m33: w.w,
        }
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

    fn zeroed() -> Self {
        [
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
        ]
        .into()
    }

    fn identity() -> Self {
        [
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::zero(),
            T::one(),
        ]
        .into()
    }

    fn get_element(&self, index: usize) -> Self::Float {
        self.as_ref()[index]
    }

    fn get_column(&self, col: Column) -> Self::Vector {
        match col {
            Column::X => Vec4::new(self.m00, self.m10, self.m20, self.m30),
            Column::Y => Vec4::new(self.m01, self.m11, self.m21, self.m31),
            Column::Z => Vec4::new(self.m02, self.m12, self.m22, self.m23),
            Column::W => Vec4::new(self.m03, self.m13, self.m23, self.m33),
        }
    }

    fn get_row(&self, row: Row) -> Self::Vector {
        match row {
            Row::X => Vec4::new(self.m00, self.m01, self.m02, self.m03),
            Row::Y => Vec4::new(self.m10, self.m11, self.m12, self.m13),
            Row::Z => Vec4::new(self.m20, self.m21, self.m22, self.m23),
            Row::W => Vec4::new(self.m30, self.m31, self.m32, self.m33),
        }
    }

    fn get_position(&self) -> Self::Vector {
        self.get_column(Column::W)
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
            m00: self.m00 + rhs.m00,
            m01: self.m01 + rhs.m01,
            m02: self.m02 + rhs.m02,
            m03: self.m03 + rhs.m03,
            m10: self.m10 + rhs.m10,
            m11: self.m11 + rhs.m11,
            m12: self.m12 + rhs.m12,
            m13: self.m13 + rhs.m13,
            m20: self.m20 + rhs.m20,
            m21: self.m21 + rhs.m21,
            m22: self.m22 + rhs.m22,
            m23: self.m23 + rhs.m23,
            m30: self.m30 + rhs.m30,
            m31: self.m31 + rhs.m31,
            m32: self.m32 + rhs.m32,
            m33: self.m33 + rhs.m33,
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
            m00: self.m00 - rhs.m00,
            m01: self.m01 - rhs.m01,
            m02: self.m02 - rhs.m02,
            m03: self.m03 - rhs.m03,
            m10: self.m10 - rhs.m10,
            m11: self.m11 - rhs.m11,
            m12: self.m12 - rhs.m12,
            m13: self.m13 - rhs.m13,
            m20: self.m20 - rhs.m20,
            m21: self.m21 - rhs.m21,
            m22: self.m22 - rhs.m22,
            m23: self.m23 - rhs.m23,
            m30: self.m30 - rhs.m30,
            m31: self.m31 - rhs.m31,
            m32: self.m32 - rhs.m32,
            m33: self.m33 - rhs.m33,
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
            m00: self.m00 * rhs,
            m01: self.m01 * rhs,
            m02: self.m02 * rhs,
            m03: self.m03 * rhs,
            m10: self.m10 * rhs,
            m11: self.m11 * rhs,
            m12: self.m12 * rhs,
            m13: self.m13 * rhs,
            m20: self.m20 * rhs,
            m21: self.m21 * rhs,
            m22: self.m22 * rhs,
            m23: self.m23 * rhs,
            m30: self.m30 * rhs,
            m31: self.m31 * rhs,
            m32: self.m32 * rhs,
            m33: self.m33 * rhs,
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
            m00: self.m00 * rhs.m00 + self.m01 * rhs.m10 + self.m02 * rhs.m20 + self.m03 * rhs.m30,
            m01: self.m00 * rhs.m01 + self.m01 * rhs.m11 + self.m02 * rhs.m21 + self.m03 * rhs.m31,
            m02: self.m00 * rhs.m02 + self.m01 * rhs.m12 + self.m02 * rhs.m22 + self.m03 * rhs.m32,
            m03: self.m00 * rhs.m03 + self.m01 * rhs.m13 + self.m02 * rhs.m23 + self.m03 * rhs.m33,
            m10: self.m10 * rhs.m00 + self.m11 * rhs.m10 + self.m12 * rhs.m20 + self.m13 * rhs.m30,
            m11: self.m10 * rhs.m01 + self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31,
            m12: self.m10 * rhs.m02 + self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32,
            m13: self.m10 * rhs.m03 + self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33,
            m20: self.m20 * rhs.m00 + self.m21 * rhs.m10 + self.m22 * rhs.m20 + self.m23 * rhs.m30,
            m21: self.m20 * rhs.m01 + self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31,
            m22: self.m20 * rhs.m02 + self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32,
            m23: self.m20 * rhs.m03 + self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33,
            m30: self.m30 * rhs.m00 + self.m31 * rhs.m10 + self.m32 * rhs.m20 + self.m33 * rhs.m30,
            m31: self.m30 * rhs.m01 + self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31,
            m32: self.m30 * rhs.m02 + self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32,
            m33: self.m30 * rhs.m03 + self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33,
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
            x: self.m00 * rhs.x + self.m01 * rhs.y + self.m02 * rhs.z + self.m03 * rhs.w,
            y: self.m10 * rhs.x + self.m11 * rhs.y + self.m12 * rhs.z + self.m13 * rhs.w,
            z: self.m20 * rhs.x + self.m21 * rhs.y + self.m22 * rhs.z + self.m23 * rhs.w,
            w: self.m30 * rhs.x + self.m31 * rhs.y + self.m32 * rhs.z + self.m33 * rhs.w,
        }
    }
}

impl<T> AsRef<[T; 16]> for Mat4<T>
where
    T: Float,
{
    fn as_ref(&self) -> &[T; 16] {
        unsafe { std::mem::transmute(self) }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Mat2 Conversions
///////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> From<[[T; 2]; 2]> for Mat2<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [[T; 2]; 2]) -> Self {
        Mat2 {
            m00: value[0][0],
            m01: value[0][1],
            m10: value[1][0],
            m11: value[1][1],
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
            m00: value[0],
            m01: value[1],
            m10: value[2],
            m11: value[3],
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Mat3 Converions
///////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> From<[[T; 3]; 3]> for Mat3<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [[T; 3]; 3]) -> Self {
        Mat3 {
            m00: value[0][0],
            m01: value[0][1],
            m02: value[0][2],
            m10: value[1][0],
            m11: value[1][1],
            m12: value[1][2],
            m20: value[2][0],
            m21: value[2][1],
            m22: value[2][2],
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
            m00: value[0],
            m01: value[1],
            m02: value[2],
            m10: value[3],
            m11: value[4],
            m12: value[5],
            m20: value[6],
            m21: value[7],
            m22: value[8],
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Mat4 Conversions
///////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> From<[[T; 4]; 4]> for Mat4<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [[T; 4]; 4]) -> Self {
        Mat4 {
            m00: value[0][0],
            m01: value[0][1],
            m02: value[0][2],
            m03: value[0][3],
            m10: value[1][0],
            m11: value[1][1],
            m12: value[1][2],
            m13: value[1][3],
            m20: value[2][0],
            m21: value[2][1],
            m22: value[2][2],
            m23: value[2][3],
            m30: value[3][0],
            m31: value[3][1],
            m32: value[3][2],
            m33: value[3][3],
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
            m00: value[0],
            m01: value[1],
            m02: value[2],
            m03: value[3],
            m10: value[4],
            m11: value[5],
            m12: value[6],
            m13: value[7],
            m20: value[8],
            m21: value[9],
            m22: value[10],
            m23: value[11],
            m30: value[12],
            m31: value[13],
            m32: value[14],
            m33: value[15],
        }
    }
}
