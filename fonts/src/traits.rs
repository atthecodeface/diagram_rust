//tp Metrics
pub trait Value:
    Clone
    + Copy
    + std::fmt::Debug
    + std::fmt::Display
    + PartialEq
    + PartialOrd
    + std::ops::Add<Output = Self>
{
    fn zero() -> Self;
}

//tp Font
impl Value for f32 {
    fn zero() -> Self {
        0.0
    }
}
impl Value for f64 {
    fn zero() -> Self {
        0.0
    }
}
