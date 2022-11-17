#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Neg, Sub};

///////////////////////////////////////////////////////////////////////////////////////////////////
// Traits
///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Float {}

impl Float for f32 {}

impl Float for f64 {}

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }
}

impl Sqrt for f64 {
    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Point2D
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Point2<T: Float> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T>
where
    T: Add<Output = T>,
    T: Float,
    T: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }
    pub fn add(&self, vector: Vec2<T>) -> Point2<T> {
        Point2 {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}

impl<T> Neg for Point2<T>
where
    T: Neg<Output = T>,
    T: Float,
{
    type Output = Point2<T>;

    fn neg(self) -> Self::Output {
        Point2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> From<[T; 2]> for Point2<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [T; 2]) -> Self {
        Point2 {
            x: value[0],
            y: value[1],
        }
    }
}

impl<T> From<(T, T)> for Point2<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: (T, T)) -> Self {
        Point2 {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<Point3<T>> for Point2<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Point3<T>) -> Self {
        Point2 {
            x: value.x,
            y: value.y,
        }
    }
}

impl<T> From<Vec2<T>> for Point2<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Vec2<T>) -> Self {
        Point2 {
            x: value.x,
            y: value.y,
        }
    }
}

impl<T> From<Vec3<T>> for Point2<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Vec3<T>) -> Self {
        Point2 {
            x: value.x,
            y: value.y,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Point3D
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Point3<T: Float> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Float,
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3 { x, y, z }
    }
    pub fn add(&self, vector: Vec3<T>) -> Point3<T> {
        Point3 {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }
}

impl<T> Neg for Point3<T>
where
    T: Neg<Output = T>,
    T: Float,
{
    type Output = Point3<T>;

    fn neg(self) -> Self::Output {
        Point3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> From<[T; 3]> for Point3<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: [T; 3]) -> Self {
        Point3 {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl<T> From<(T, T, T)> for Point3<T>
where
    T: Float,
    T: Copy,
    T: Clone,
{
    fn from(value: (T, T, T)) -> Self {
        Point3 {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl<T> From<Point2<T>> for Point3<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Point2<T>) -> Self {
        Point3 {
            x: value.x,
            y: value.y,
            z: T::default(),
        }
    }
}

impl<T> From<Vec2<T>> for Point3<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Vec2<T>) -> Self {
        Point3 {
            x: value.x,
            y: value.y,
            z: T::default(),
        }
    }
}

impl<T> From<Vec3<T>> for Point3<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Vec3<T>) -> Self {
        Point3 {
            x: value.x,
            y: value.y,
            z: value.z,
        }
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
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
    T: Float,
    T: Sqrt,
    T: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    pub fn add(&self, vector: Vec2<T>) -> Self {
        Vec2 {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }

    pub fn sub(&self, vector: Vec2<T>) -> Self {
        Vec2 {
            x: self.x - vector.x,
            y: self.y - vector.y,
        }
    }

    pub fn scale(&self, scalar: T) -> Self {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    pub fn magnitude(&self) -> T {
        T::sqrt(&((self.x * self.x) + (self.y * self.y)))
    }

    pub fn normalize(&self) -> Vec2<T> {
        let mag = self.magnitude();
        Vec2 {
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

impl<T> From<Point2<T>> for Vec2<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Point2<T>) -> Self {
        Vec2 {
            x: value.x,
            y: value.y,
        }
    }
}

impl<T> From<Point3<T>> for Vec2<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Point3<T>) -> Self {
        Vec2 {
            x: value.x,
            y: value.y,
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
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Div<Output = T>,
    T: Float,
    T: Sqrt,
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn add(&self, vector: Vec3<T>) -> Self {
        Vec3 {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }

    pub fn sub(&self, vector: Vec3<T>) -> Self {
        Vec3 {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        }
    }

    pub fn scale(&self, scalar: T) -> Self {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn magnitude(&self) -> T {
        T::sqrt(&((self.x * self.x) + (self.y * self.y) + (self.z * self.z)))
    }

    pub fn normalize(&self) -> Vec3<T> {
        let mag = self.magnitude();
        Vec3 {
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

impl<T> From<Point2<T>> for Vec3<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Point2<T>) -> Self {
        Vec3 {
            x: value.x,
            y: value.y,
            z: T::default(),
        }
    }
}

impl<T> From<Point3<T>> for Vec3<T>
where
    T: Float,
    T: Default,
{
    fn from(value: Point3<T>) -> Self {
        Vec3 {
            x: value.x,
            y: value.y,
            z: value.z,
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
