use super::*;
use crate::Zero;
use std::ops::Neg;

macro_rules! impl_rijk_neg {
    ($t:ident) => {
        impl<T: Neg> Neg for $t<T> {
            type Output = $t<T::Output>;

            fn neg(self) -> Self::Output {
                $t(self.0.neg())
            }
        }
    };
}

impl_rijk_neg!(R);
impl_rijk_neg!(I);
impl_rijk_neg!(J);
impl_rijk_neg!(K);

macro_rules! impl_is_neg_rijk {
    ($t:ident) => {
        impl<T> $t<T>
        where
            T: Zero + PartialOrd,
        {
            pub fn is_neg(&self) -> bool {
                *self < $t::zero()
            }
        }
    };
}

impl_is_neg_rijk!(R);
impl_is_neg_rijk!(I);
impl_is_neg_rijk!(J);
impl_is_neg_rijk!(K);

impl<T: Neg> Neg for Quaternion<T> {
    type Output = Quaternion<T::Output>;

    fn neg(self) -> Self::Output {
        Quaternion {
            r: self.r.neg(),
            i: self.i.neg(),
            j: self.j.neg(),
            k: self.k.neg(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn rijk_neg_test() {
        assert_eq!(-r(1), r(-1));
        assert_eq!(-i(1), i(-1));
        assert_eq!(-j(1), j(-1));
        assert_eq!(-k(1), k(-1));
    }

    #[test]
    fn quaternion_neg_test() {
        assert_eq!(-quaternion(1, 2, 3, 4), quaternion(-1, -2, -3, -4));
    }
}
