use std::{
    cmp::PartialEq,
    fmt::Display,
    ops::{AddAssign, MulAssign, SubAssign},
};

///////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
///////////////////////////////////////////////////////////////////////////////////////////////////

pub const RADIAN: f64 = 57.2957795131;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Traits
///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Float: Display + Copy + AddAssign + SubAssign + MulAssign + PartialEq {
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn sin_cos(self) -> (Self, Self)
    where
        Self: Sized;
    fn acos(self) -> Self;
    fn one() -> Self;
    fn zero() -> Self;
    fn neg_one() -> Self;
    fn one_half() -> Self;

    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;

    fn aprox_eq(self, other: Self, within: Self) -> bool;
}

impl Float for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }

    fn sin(self) -> Self {
        f32::sin(self)
    }

    fn cos(self) -> Self {
        f32::cos(self)
    }

    fn sin_cos(self) -> (Self, Self) {
        f32::sin_cos(self)
    }

    fn acos(self) -> Self {
        f32::acos(self)
    }

    fn one() -> Self {
        1.0f32
    }

    fn zero() -> Self {
        0.0f32
    }

    fn neg_one() -> Self {
        -1.0f32
    }

    fn one_half() -> Self {
        0.5f32
    }

    fn to_degrees(self) -> Self {
        f32::to_degrees(self)
    }

    fn to_radians(self) -> Self {
        f32::to_radians(self)
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        f32::clamp(self, min, max)
    }

    fn aprox_eq(self, other: Self, within: Self) -> bool {
        (self - other) <= within
    }
}

impl Float for f64 {
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }

    fn sin(self) -> Self {
        f64::sin(self)
    }

    fn cos(self) -> Self {
        f64::cos(self)
    }

    fn sin_cos(self) -> (Self, Self) {
        f64::sin_cos(self)
    }

    fn acos(self) -> Self {
        f64::acos(self)
    }

    fn one() -> Self {
        1.0f64
    }

    fn zero() -> Self {
        0.0f64
    }

    fn neg_one() -> Self {
        -1.0f64
    }

    fn one_half() -> Self {
        0.5f64
    }

    fn to_degrees(self) -> Self {
        f64::to_degrees(self)
    }

    fn to_radians(self) -> Self {
        f64::to_radians(self)
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        f64::clamp(self, min, max)
    }

    fn aprox_eq(self, other: Self, within: Self) -> bool {
        (self - other) <= within
    }
}
