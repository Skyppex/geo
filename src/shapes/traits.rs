pub trait Pi<Output = Self> {
    type Output;
    
    #[inline]
    fn pi() -> Self::Output;
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