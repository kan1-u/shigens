use super::*;
use crate::{One, Zero};
use std::ops::{Add, Div, Mul, Neg};

macro_rules! impl_zero_rijk {
    ($t:ident) => {
        impl<T> Zero for $t<T>
        where
            T: Zero,
        {
            fn zero() -> Self {
                $t(T::zero())
            }
        }
    };
}

impl_zero_rijk!(R);
impl_zero_rijk!(I);
impl_zero_rijk!(J);
impl_zero_rijk!(K);

impl<T> Zero for Quaternion<T>
where
    T: Zero,
{
    fn zero() -> Self {
        Quaternion {
            r: R::zero(),
            i: I::zero(),
            j: J::zero(),
            k: K::zero(),
        }
    }
}

impl<T> One for R<T>
where
    T: One,
{
    fn one() -> Self {
        R(T::one())
    }
}

impl<T> One for Quaternion<T>
where
    T: One + Zero,
{
    fn one() -> Self {
        Quaternion {
            r: R::one(),
            i: I::zero(),
            j: J::zero(),
            k: K::zero(),
        }
    }
}

macro_rules! impl_get_value_rijk {
    ($t:ident) => {
        impl<T> $t<T> {
            pub fn get_value(self) -> T {
                self.0
            }
        }
    };
}

impl_get_value_rijk!(R);
impl_get_value_rijk!(I);
impl_get_value_rijk!(J);
impl_get_value_rijk!(K);

impl<T> Quaternion<T> {
    pub fn get_values(self) -> (T, T, T, T) {
        (self.r.0, self.i.0, self.j.0, self.k.0)
    }
}

impl<T> Quaternion<T>
where
    T: Copy + Neg<Output = T>,
{
    pub fn rev(&self) -> Quaternion<T> {
        Quaternion {
            r: self.r,
            i: self.i.neg(),
            j: self.j.neg(),
            k: self.k.neg(),
        }
    }
}

impl<T> Quaternion<T>
where
    T: Copy + Mul,
    T::Output: Add<Output = T::Output>,
{
    pub fn norm_square(&self) -> R<T::Output> {
        R(self.r.0 * self.r.0 + self.i.0 * self.i.0 + self.j.0 * self.j.0 + self.k.0 * self.k.0)
    }
}

impl<T> Quaternion<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T>,
{
    pub fn inv(&self) -> Quaternion<T> {
        self.rev() / self.norm_square()
    }
}

impl<T> Quaternion<T> {
    pub fn scalar(self) -> R<T> {
        self.r
    }

    pub fn vector(self) -> (I<T>, J<T>, K<T>) {
        (self.i, self.j, self.k)
    }
}
