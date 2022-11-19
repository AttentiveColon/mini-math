use std::fmt::Display;
///////////////////////////////////////////////////////////////////////////////////////////////////
// Traits
///////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Float: Display {
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
