mod funcs;
mod neg;
mod num;
mod ops;

pub use funcs::*;
pub use neg::*;
pub use num::*;
pub use ops::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct R<T>(pub(crate) T);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct I<T>(pub(crate) T);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct J<T>(pub(crate) T);
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct K<T>(pub(crate) T);

#[derive(Debug, Clone, PartialEq)]
pub struct Quaternion<T> {
    pub r: R<T>,
    pub i: I<T>,
    pub j: J<T>,
    pub k: K<T>,
}

macro_rules! impl_from_rijk {
    ($t:ident) => {
        impl<T> From<T> for $t<T> {
            fn from(x: T) -> Self {
                Self(x)
            }
        }
    };
}

impl_from_rijk!(R);
impl_from_rijk!(I);
impl_from_rijk!(J);
impl_from_rijk!(K);

impl<T> From<(T, T, T, T)> for Quaternion<T> {
    fn from((r, i, j, k): (T, T, T, T)) -> Self {
        Self {
            r: R(r),
            i: I(i),
            j: J(j),
            k: K(k),
        }
    }
}

impl<T: fmt::Display> fmt::Display for R<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! impl_display_rijk {
    ($T:ident,$f:expr) => {
        impl<T: fmt::Display> fmt::Display for $T<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, $f, self.0)
            }
        }
    };
}

impl_display_rijk!(I, "{}i");
impl_display_rijk!(J, "{}j");
impl_display_rijk!(K, "{}k");

impl<T: fmt::Display> fmt::Display for Quaternion<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}) + ({}) + ({}) + ({})",
            self.r, self.i, self.j, self.k
        )
    }
}
