pub trait Zero {
    fn zero() -> Self;

    fn is_zero(&self) -> bool
    where
        Self: Sized + PartialEq,
    {
        *self == Self::zero()
    }
}

macro_rules! impl_zero_int {
    ($t:ty) => {
        impl Zero for $t {
            fn zero() -> Self {
                0
            }
        }
    };
}

impl_zero_int!(isize);
impl_zero_int!(i8);
impl_zero_int!(i16);
impl_zero_int!(i32);
impl_zero_int!(i64);
impl_zero_int!(i128);
impl_zero_int!(usize);
impl_zero_int!(u8);
impl_zero_int!(u16);
impl_zero_int!(u32);
impl_zero_int!(u64);
impl_zero_int!(u128);

macro_rules! impl_zero_float {
    ($t:ty) => {
        impl Zero for $t {
            fn zero() -> Self {
                0.
            }
        }
    };
}

impl_zero_float!(f32);
impl_zero_float!(f64);
