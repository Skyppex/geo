use num_traits::Float;

#[cfg(feature = "half")]
use half::{f16, bf16};

pub trait Pi<Output = Self> {
    type Output: Float;
    
    #[inline]
    fn pi() -> Self::Output;
}

#[cfg(feature = "half")]
impl Pi for f16 {
    type Output = f16;

    fn pi() -> Self::Output {
        f16::from_f32(3.141592653589793238462643383279502884)
    }
}

#[cfg(feature = "half")]
impl Pi for bf16 {
    type Output = bf16;

    fn pi() -> Self::Output {
        bf16::from_f32(3.141592653589793238462643383279502884)
    }
}

impl Pi for f32 {
    type Output = f32;
    
    #[inline]
    fn pi() -> f32 {
        3.141592653589793238462643383279502884
    }
}

impl Pi for f64 {
    type Output = f64;
    
    #[inline]
    fn pi() -> f64 {
        3.141592653589793238462643383279502884
    }
}
