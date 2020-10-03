use super::*;
use crate::Zero;
use std::ops::{Add, Div, Mul, Neg, Sub};

macro_rules! impl_rijk_add_sub_same {
    ($t:ident) => {
        macro_rules! impl_ops {
            ($O:ident,$o:ident) => {
                impl<LT, RT> $O<$t<RT>> for $t<LT>
                where
                    LT: $O<RT>,
                {
                    type Output = $t<LT::Output>;

                    fn $o(self, rhs: $t<RT>) -> Self::Output {
                        $t((self.0).$o(rhs.0))
                    }
                }
            };
        }

        impl_ops!(Add, add);
        impl_ops!(Sub, sub);
    };
}

impl_rijk_add_sub_same!(R);
impl_rijk_add_sub_same!(I);
impl_rijk_add_sub_same!(J);
impl_rijk_add_sub_same!(K);

macro_rules! impl_rijk_add_sub_diff {
    ($L:ident,$l:ident,$R:ident,$r:ident,$O1:ident,$o1:ident,$O2:ident,$o2:ident) => {
        impl<T> Add<$R<T>> for $L<T>
        where
            T: Zero,
        {
            type Output = Quaternion<T>;

            fn add(self, rhs: $R<T>) -> Self::Output {
                Quaternion {
                    $l: self,
                    $r: rhs,
                    $o1: $O1::zero(),
                    $o2: $O2::zero(),
                }
            }
        }

        impl<T> Sub<$R<T>> for $L<T>
        where
            T: Zero + Neg<Output = T>,
        {
            type Output = Quaternion<T>;

            fn sub(self, rhs: $R<T>) -> Self::Output {
                Quaternion {
                    $l: self,
                    $r: rhs.neg(),
                    $o1: $O1::zero(),
                    $o2: $O2::zero(),
                }
            }
        }
    };
}

impl_rijk_add_sub_diff!(R, r, I, i, J, j, K, k);
impl_rijk_add_sub_diff!(R, r, J, j, I, i, K, k);
impl_rijk_add_sub_diff!(R, r, K, k, I, i, J, j);
impl_rijk_add_sub_diff!(I, i, R, r, J, j, K, k);
impl_rijk_add_sub_diff!(I, i, J, j, R, r, K, k);
impl_rijk_add_sub_diff!(I, i, K, k, R, r, J, j);
impl_rijk_add_sub_diff!(J, j, R, r, I, i, K, k);
impl_rijk_add_sub_diff!(J, j, I, i, R, r, K, k);
impl_rijk_add_sub_diff!(J, j, K, k, R, r, I, i);
impl_rijk_add_sub_diff!(K, k, R, r, I, i, J, j);
impl_rijk_add_sub_diff!(K, k, I, i, R, r, J, j);
impl_rijk_add_sub_diff!(K, k, J, j, R, r, I, i);

macro_rules! impl_rijk_mul_rijk_pos {
    ($l:ident,$r:ident,$o:ident) => {
        impl<LT, RT> Mul<$r<RT>> for $l<LT>
        where
            LT: Mul<RT>,
        {
            type Output = $o<LT::Output>;

            fn mul(self, rhs: $r<RT>) -> Self::Output {
                $o(self.0.mul(rhs.0))
            }
        }
    };
}

macro_rules! impl_rijk_mul_rijk_neg {
    ($l:ident,$r:ident,$o:ident) => {
        impl<LT, RT> Mul<$r<RT>> for $l<LT>
        where
            LT: Mul<RT>,
            LT::Output: Neg,
        {
            type Output = $o<<LT::Output as Neg>::Output>;

            fn mul(self, rhs: $r<RT>) -> Self::Output {
                $o(self.0.mul(rhs.0).neg())
            }
        }
    };
}

impl_rijk_mul_rijk_pos!(R, R, R);
impl_rijk_mul_rijk_pos!(R, I, I);
impl_rijk_mul_rijk_pos!(R, J, J);
impl_rijk_mul_rijk_pos!(R, K, K);
impl_rijk_mul_rijk_pos!(I, R, I);
impl_rijk_mul_rijk_pos!(J, R, J);
impl_rijk_mul_rijk_pos!(K, R, K);

impl_rijk_mul_rijk_neg!(I, I, R);
impl_rijk_mul_rijk_neg!(J, J, R);
impl_rijk_mul_rijk_neg!(K, K, R);

impl_rijk_mul_rijk_pos!(I, J, K);
impl_rijk_mul_rijk_pos!(J, K, I);
impl_rijk_mul_rijk_pos!(K, I, J);

impl_rijk_mul_rijk_neg!(J, I, K);
impl_rijk_mul_rijk_neg!(K, J, I);
impl_rijk_mul_rijk_neg!(I, K, J);

macro_rules! impl_rijk_div_rijk_pos {
    ($l:ident,$r:ident,$y:ident) => {
        impl<LT, RT> Div<$r<RT>> for $l<LT>
        where
            LT: Div<RT>,
        {
            type Output = $y<LT::Output>;

            fn div(self, rhs: $r<RT>) -> Self::Output {
                $y(self.0.div(rhs.0))
            }
        }
    };
}

macro_rules! impl_rijk_div_rijk_neg {
    ($l:ident,$r:ident,$y:ident) => {
        impl<LT, RT> Div<$r<RT>> for $l<LT>
        where
            LT: Div<RT>,
            LT::Output: Neg,
        {
            type Output = $y<<LT::Output as Neg>::Output>;

            fn div(self, rhs: $r<RT>) -> Self::Output {
                $y(self.0.div(rhs.0).neg())
            }
        }
    };
}

impl_rijk_div_rijk_pos!(R, R, R);
impl_rijk_div_rijk_neg!(R, I, I);
impl_rijk_div_rijk_neg!(R, J, J);
impl_rijk_div_rijk_neg!(R, K, K);
impl_rijk_div_rijk_pos!(I, R, I);
impl_rijk_div_rijk_pos!(J, R, J);
impl_rijk_div_rijk_pos!(K, R, K);

impl_rijk_div_rijk_pos!(I, I, R);
impl_rijk_div_rijk_pos!(J, J, R);
impl_rijk_div_rijk_pos!(K, K, R);

impl_rijk_div_rijk_neg!(I, J, K);
impl_rijk_div_rijk_neg!(J, K, I);
impl_rijk_div_rijk_neg!(K, I, J);

impl_rijk_div_rijk_pos!(J, I, K);
impl_rijk_div_rijk_pos!(K, J, I);
impl_rijk_div_rijk_pos!(I, K, J);

macro_rules! impl_quaternion_add_or_sub_quaternion {
    ($O:ident,$o:ident) => {
        impl<LT, RT> $O<Quaternion<RT>> for Quaternion<LT>
        where
            LT: $O<RT>,
        {
            type Output = Quaternion<LT::Output>;

            fn $o(self, rhs: Quaternion<RT>) -> Self::Output {
                Quaternion {
                    r: (self.r).$o(rhs.r),
                    i: (self.i).$o(rhs.i),
                    j: (self.j).$o(rhs.j),
                    k: (self.k).$o(rhs.k),
                }
            }
        }
    };
}

impl_quaternion_add_or_sub_quaternion!(Add, add);
impl_quaternion_add_or_sub_quaternion!(Sub, sub);

impl<LT, RT> Mul<Quaternion<RT>> for Quaternion<LT>
where
    LT: Copy + Mul<RT>,
    RT: Copy,
    LT::Output: Add<Output = LT::Output> + Neg<Output = LT::Output>,
{
    type Output = Quaternion<LT::Output>;

    fn mul(self, rhs: Quaternion<RT>) -> Self::Output {
        Quaternion {
            r: self.r * rhs.r + self.i * rhs.i + self.j * rhs.j + self.k * rhs.k,
            i: self.r * rhs.i + self.i * rhs.r + self.j * rhs.k + self.k * rhs.j,
            j: self.r * rhs.j + self.i * rhs.k + self.j * rhs.r + self.k * rhs.i,
            k: self.r * rhs.k + self.i * rhs.j + self.j * rhs.i + self.k * rhs.r,
        }
    }
}

macro_rules! impl_quaternion_add_sub_quaternion_rijk {
    ($T:ident,$t:ident,$o1:ident,$o2:ident,$o3:ident) => {
        macro_rules! impl_ops {
            ($O:ident,$o:ident) => {
                impl<T> $O<$T<T>> for Quaternion<T>
                where
                    T: $O<Output = T>,
                {
                    type Output = Quaternion<T>;

                    fn $o(self, rhs: $T<T>) -> Self::Output {
                        Quaternion {
                            $t: self.$t.$o(rhs),
                            ..self
                        }
                    }
                }
            };
        }

        impl_ops!(Add, add);
        impl_ops!(Sub, sub);

        impl<T> Add<Quaternion<T>> for $T<T>
        where
            T: Add<Output = T>,
        {
            type Output = Quaternion<T>;

            fn add(self, rhs: Quaternion<T>) -> Self::Output {
                Quaternion {
                    $t: self.add(rhs.$t),
                    ..rhs
                }
            }
        }

        impl<T> Sub<Quaternion<T>> for $T<T>
        where
            T: Sub<Output = T> + Neg<Output = T>,
        {
            type Output = Quaternion<T>;

            fn sub(self, rhs: Quaternion<T>) -> Self::Output {
                Quaternion {
                    $t: self.sub(rhs.$t),
                    $o1: rhs.$o1.neg(),
                    $o2: rhs.$o2.neg(),
                    $o3: rhs.$o3.neg(),
                }
            }
        }
    };
}

impl_quaternion_add_sub_quaternion_rijk!(R, r, i, j, k);
impl_quaternion_add_sub_quaternion_rijk!(I, i, r, j, k);
impl_quaternion_add_sub_quaternion_rijk!(J, j, r, i, k);
impl_quaternion_add_sub_quaternion_rijk!(K, k, r, i, j);

macro_rules! impl_quaternion_mul_div_r {
    ($O:ident,$o:ident) => {
        impl<T> $O<R<T>> for Quaternion<T>
        where
            T: Copy + $O,
        {
            type Output = Quaternion<T::Output>;

            fn $o(self, rhs: R<T>) -> Self::Output {
                Quaternion {
                    r: self.r.$o(rhs),
                    i: self.i.$o(rhs),
                    j: self.j.$o(rhs),
                    k: self.k.$o(rhs),
                }
            }
        }
    };
}

impl_quaternion_mul_div_r!(Mul, mul);
impl_quaternion_mul_div_r!(Div, div);

impl<T> Mul<Quaternion<T>> for R<T>
where
    T: Copy + Mul,
{
    type Output = Quaternion<T::Output>;

    fn mul(self, rhs: Quaternion<T>) -> Self::Output {
        Quaternion {
            r: self.mul(rhs.r),
            i: self.mul(rhs.i),
            j: self.mul(rhs.j),
            k: self.mul(rhs.k),
        }
    }
}

macro_rules! impl_quaternion_mul_div_ijk {
    ($t:ident,$r:ident,$i:ident,$j:ident,$k:ident) => {
        macro_rules! impl_ops {
            ($O:ident,$o:ident) => {
                impl<T> $O<$t<T>> for Quaternion<T>
                where
                    T: Copy + $O<Output = T> + Neg<Output = T>,
                {
                    type Output = Quaternion<T>;

                    fn $o(self, rhs: $t<T>) -> Self::Output {
                        Quaternion {
                            r: self.$r.$o(rhs),
                            i: self.$i.$o(rhs),
                            j: self.$j.$o(rhs),
                            k: self.$k.$o(rhs),
                        }
                    }
                }
            };
        }

        impl_ops!(Mul, mul);
        impl_ops!(Div, div);

        impl<T> Mul<Quaternion<T>> for $t<T>
        where
            T: Copy + Mul<Output = T> + Neg<Output = T>,
        {
            type Output = Quaternion<T>;

            fn mul(self, rhs: Quaternion<T>) -> Self::Output {
                Quaternion {
                    r: self.mul(rhs.$r),
                    i: self.mul(rhs.$i),
                    j: self.mul(rhs.$j),
                    k: self.mul(rhs.$k),
                }
            }
        }
    };
}

impl_quaternion_mul_div_ijk!(I, i, r, k, j);
impl_quaternion_mul_div_ijk!(J, j, k, r, i);
impl_quaternion_mul_div_ijk!(K, k, j, i, r);

macro_rules! impl_rijk_quaternion_div_quaternion {
    ($t:ident) => {
        impl<T> Div<Quaternion<T>> for $t<T>
        where
            T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Neg<Output = T>,
        {
            type Output = Quaternion<T>;

            fn div(self, rhs: Quaternion<T>) -> Self::Output {
                self * rhs.rev() / rhs.norm_square()
            }
        }
    };
}

impl_rijk_quaternion_div_quaternion!(R);
impl_rijk_quaternion_div_quaternion!(I);
impl_rijk_quaternion_div_quaternion!(J);
impl_rijk_quaternion_div_quaternion!(K);
impl_rijk_quaternion_div_quaternion!(Quaternion);

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn rijk_add_rijk_test() {
        assert_eq!(r(1) + r(2), r(3));
        assert_eq!(i(1) + i(2), i(3));
        assert_eq!(j(1) + j(2), j(3));
        assert_eq!(k(1) + k(2), k(3));
        assert_eq!(r(1) + i(2), quaternion(1, 2, 0, 0));
        assert_eq!(r(1) + j(2), quaternion(1, 0, 2, 0));
        assert_eq!(r(1) + k(2), quaternion(1, 0, 0, 2));
        assert_eq!(i(1) + r(2), quaternion(2, 1, 0, 0));
        assert_eq!(i(1) + j(2), quaternion(0, 1, 2, 0));
        assert_eq!(i(1) + k(2), quaternion(0, 1, 0, 2));
        assert_eq!(j(1) + r(2), quaternion(2, 0, 1, 0));
        assert_eq!(j(1) + i(2), quaternion(0, 2, 1, 0));
        assert_eq!(j(1) + k(2), quaternion(0, 0, 1, 2));
        assert_eq!(k(1) + r(2), quaternion(2, 0, 0, 1));
        assert_eq!(k(1) + i(2), quaternion(0, 2, 0, 1));
        assert_eq!(k(1) + j(2), quaternion(0, 0, 2, 1));
    }

    #[test]
    fn rijk_sub_rijk_test() {
        assert_eq!(r(2) - r(1), r(1));
        assert_eq!(i(2) - i(1), i(1));
        assert_eq!(j(2) - j(1), j(1));
        assert_eq!(k(2) - k(1), k(1));
        assert_eq!(r(2) - i(1), quaternion(2, -1, 0, 0));
        assert_eq!(r(2) - j(1), quaternion(2, 0, -1, 0));
        assert_eq!(r(2) - k(1), quaternion(2, 0, 0, -1));
        assert_eq!(i(2) - r(1), quaternion(-1, 2, 0, 0));
        assert_eq!(i(2) - j(1), quaternion(0, 2, -1, 0));
        assert_eq!(i(2) - k(1), quaternion(0, 2, 0, -1));
        assert_eq!(j(2) - r(1), quaternion(-1, 0, 2, 0));
        assert_eq!(j(2) - i(1), quaternion(0, -1, 2, 0));
        assert_eq!(j(2) - k(1), quaternion(0, 0, 2, -1));
        assert_eq!(k(2) - r(1), quaternion(-1, 0, 0, 2));
        assert_eq!(k(2) - i(1), quaternion(0, -1, 0, 2));
        assert_eq!(k(2) - j(1), quaternion(0, 0, -1, 2));
    }

    #[test]
    fn rijk_mul_rijk_test() {
        assert_eq!(r(2) * r(3), r(6));
        assert_eq!(r(2) * i(3), i(6));
        assert_eq!(r(2) * j(3), j(6));
        assert_eq!(r(2) * k(3), k(6));
        assert_eq!(i(2) * r(3), i(6));
        assert_eq!(j(2) * r(3), j(6));
        assert_eq!(k(2) * r(3), k(6));
        assert_eq!(i(2) * i(3), r(-6));
        assert_eq!(j(2) * j(3), r(-6));
        assert_eq!(k(2) * k(3), r(-6));
        assert_eq!(i(2) * j(3), k(6));
        assert_eq!(j(2) * k(3), i(6));
        assert_eq!(k(2) * i(3), j(6));
        assert_eq!(j(2) * i(3), k(-6));
        assert_eq!(k(2) * j(3), i(-6));
        assert_eq!(i(2) * k(3), j(-6));
        assert_eq!(i(1) * i(1), r(-1));
    }

    #[test]
    fn rijk_div_rijk_test() {
        assert_eq!(r(6) / r(3), r(2));
        assert_eq!(r(6) / i(3), i(-2));
        assert_eq!(r(6) / j(3), j(-2));
        assert_eq!(r(6) / k(3), k(-2));
        assert_eq!(i(6) / r(3), i(2));
        assert_eq!(j(6) / r(3), j(2));
        assert_eq!(k(6) / r(3), k(2));
        assert_eq!(i(6) / i(3), r(2));
        assert_eq!(j(6) / j(3), r(2));
        assert_eq!(k(6) / k(3), r(2));
        assert_eq!(i(6) / j(3), k(-2));
        assert_eq!(j(6) / k(3), i(-2));
        assert_eq!(k(6) / i(3), j(-2));
        assert_eq!(j(6) / i(3), k(2));
        assert_eq!(k(6) / j(3), i(2));
        assert_eq!(i(6) / k(3), j(2));
    }

    #[test]
    fn quaternion_add_quaternion_test() {
        assert_eq!(
            quaternion(1, 2, 3, 4) + quaternion(5, 6, 7, 8),
            quaternion(6, 8, 10, 12)
        );
    }

    #[test]
    fn quaternion_sub_quaternion_test() {
        assert_eq!(
            quaternion(5, 6, 7, 8) - quaternion(4, 3, 2, 1),
            quaternion(1, 3, 5, 7)
        );
    }

    #[test]
    fn quaternion_mul_quaternion_test() {
        let (r1, i1, j1, k1) = (2, 3, 4, 5);
        let (r2, i2, j2, k2) = (5, 4, 3, 2);
        assert_eq!(
            quaternion(r1, i1, j1, k1) * quaternion(r2, i2, j2, k2),
            quaternion(
                r1 * r2 - i1 * i2 - j1 * j2 - k1 * k2,
                r1 * i2 + i1 * r2 + j1 * k2 - k1 * j2,
                r1 * j2 + j1 * r2 + k1 * i2 - i1 * k2,
                r1 * k2 + k1 * r2 + i1 * j2 - j1 * i2
            )
        );
    }

    #[test]
    fn quaternion_div_quaternion_test() {
        let (r1, i1, j1, k1) = (2., 3., 4., 5.);
        let (r2, i2, j2, k2) = (5., 4., 3., 2.);
        let norm_square = r2 * r2 + i2 * i2 + j2 * j2 + k2 * k2;
        let (r2r, i2r, j2r, k2r) = (r2, -i2, -j2, -k2);
        assert_eq!(
            quaternion(r1, i1, j1, k1) / quaternion(r2, i2, j2, k2),
            quaternion(
                (r1 * r2r - i1 * i2r - j1 * j2r - k1 * k2r) / norm_square,
                (r1 * i2r + i1 * r2r + j1 * k2r - k1 * j2r) / norm_square,
                (r1 * j2r + j1 * r2r + k1 * i2r - i1 * k2r) / norm_square,
                (r1 * k2r + k1 * r2r + i1 * j2r - j1 * i2r) / norm_square
            )
        );
    }

    #[test]
    fn quaternion_add_rijk_test() {
        assert_eq!(quaternion(1, 2, 3, 4) + r(5), quaternion(6, 2, 3, 4));
        assert_eq!(quaternion(1, 2, 3, 4) + i(5), quaternion(1, 7, 3, 4));
        assert_eq!(quaternion(1, 2, 3, 4) + j(5), quaternion(1, 2, 8, 4));
        assert_eq!(quaternion(1, 2, 3, 4) + k(5), quaternion(1, 2, 3, 9));
    }

    #[test]
    fn quaternion_sub_rijk_test() {
        assert_eq!(quaternion(5, 4, 3, 2) - r(1), quaternion(4, 4, 3, 2));
        assert_eq!(quaternion(5, 4, 3, 2) - i(1), quaternion(5, 3, 3, 2));
        assert_eq!(quaternion(5, 4, 3, 2) - j(1), quaternion(5, 4, 2, 2));
        assert_eq!(quaternion(5, 4, 3, 2) - k(1), quaternion(5, 4, 3, 1));
    }

    #[test]
    fn quaternion_mul_rijk_test() {
        assert_eq!(quaternion(1, 2, 3, 4) * r(2), quaternion(2, 4, 6, 8));
        assert_eq!(quaternion(1, 2, 3, 4) * i(2), quaternion(-4, 2, 8, -6));
        assert_eq!(quaternion(1, 2, 3, 4) * j(2), quaternion(-6, -8, 2, 4));
        assert_eq!(quaternion(1, 2, 3, 4) * k(2), quaternion(-8, 6, -4, 2));
    }

    #[test]
    fn quaternion_div_rijk_test() {
        assert_eq!(quaternion(2, 4, 6, 8) / r(2), quaternion(1, 2, 3, 4));
        assert_eq!(quaternion(2, 4, 6, 8) / i(2), quaternion(2, -1, -4, 3));
        assert_eq!(quaternion(2, 4, 6, 8) / j(2), quaternion(3, 4, -1, -2));
        assert_eq!(quaternion(2, 4, 6, 8) / k(2), quaternion(4, -3, 2, -1));
    }

    #[test]
    fn rijk_add_quaternion_test() {
        assert_eq!(r(5) + quaternion(1, 2, 3, 4), quaternion(6, 2, 3, 4));
        assert_eq!(i(5) + quaternion(1, 2, 3, 4), quaternion(1, 7, 3, 4));
        assert_eq!(j(5) + quaternion(1, 2, 3, 4), quaternion(1, 2, 8, 4));
        assert_eq!(k(5) + quaternion(1, 2, 3, 4), quaternion(1, 2, 3, 9));
    }

    #[test]
    fn rijk_sub_quaternion_test() {
        assert_eq!(r(5) - quaternion(4, 3, 2, 1), quaternion(1, -3, -2, -1));
        assert_eq!(i(5) - quaternion(4, 3, 2, 1), quaternion(-4, 2, -2, -1));
        assert_eq!(j(5) - quaternion(4, 3, 2, 1), quaternion(-4, -3, 3, -1));
        assert_eq!(k(5) - quaternion(4, 3, 2, 1), quaternion(-4, -3, -2, 4));
    }

    #[test]
    fn rijk_mul_quaternion_test() {
        assert_eq!(r(2) * quaternion(1, 2, 3, 4), quaternion(2, 4, 6, 8));
        assert_eq!(i(2) * quaternion(1, 2, 3, 4), quaternion(-4, 2, -8, 6));
        assert_eq!(j(2) * quaternion(1, 2, 3, 4), quaternion(-6, 8, 2, -4));
        assert_eq!(k(2) * quaternion(1, 2, 3, 4), quaternion(-8, -6, 4, 2));
    }

    #[test]
    fn rijk_div_quaternion_test() {
        assert_eq!(r(30) / quaternion(1, 2, 3, 4), quaternion(1, -2, -3, -4));
        assert_eq!(i(30) / quaternion(1, 2, 3, 4), quaternion(2, 1, 4, -3));
        assert_eq!(j(30) / quaternion(1, 2, 3, 4), quaternion(3, -4, 1, 2));
        assert_eq!(k(30) / quaternion(1, 2, 3, 4), quaternion(4, 3, -2, 1));
    }
}
