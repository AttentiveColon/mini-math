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

    fn zeroed() -> Self {
        todo!()
    }

    fn identity() -> Self {
        todo!()
    }

    fn get_element(&self, _index: usize) -> Self::Float {
        todo!()
    }

    fn get_column(&self, _col: Column) -> Self::Vector {
        todo!()
    }

    fn get_row(&self, _row: Row) -> Self::Vector {
        todo!()
    }

    fn get_position(&self) -> Self::Vector {
        todo!()
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
