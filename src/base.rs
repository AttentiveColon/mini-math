use std::fmt::Display;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
///////////////////////////////////////////////////////////////////////////////////////////////////

pub const RADIAN: f64 = 57.2957795131;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Traits
///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Float: Display {
    fn sqrt(self) -> Self;
    fn acos(self) -> Self;
    fn one() -> Self;
    fn zero() -> Self;

    fn in_deg(self) -> Self;
}

impl Float for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
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

    fn in_deg(self) -> Self {
        self * RADIAN as f32
    }
}

impl Float for f64 {
    fn sqrt(self) -> Self {
        f64::sqrt(self)
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

    fn in_deg(self) -> Self {
        self * RADIAN
    }
}
