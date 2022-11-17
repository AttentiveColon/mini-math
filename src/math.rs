#![allow(dead_code)]

use std::ops::{Add, Sub};

///////////////////////////////////////////////////////////////////////////////////////////////////
// Point2D
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, PartialOrd)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }
    pub fn add(&self, point: Point2<T>) -> Point2<T> {
        Point2 {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
    pub fn sub(&self, point: Point2<T>) -> Point2<T> {
        Point2 {
            x: self.x - point.x,
            y: self.y - point.y,
        }
    }
}

impl<T> From<[T; 2]> for Point2<T>
where
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
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3 { x, y, z }
    }
}

impl<T> From<[T; 3]> for Point3<T>
where
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
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl<T> From<[T; 2]> for Vec2<T>
where
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
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T> From<[T; 3]> for Vec3<T>
where
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
