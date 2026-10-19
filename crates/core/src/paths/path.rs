use enum_dispatch::enum_dispatch;
use num::complex::Complex;
use crate::paths::{PointPath, SVGPath, SegmentPath};

#[enum_dispatch]
/// Trait which must be implemented by all Path structs.
pub trait PathTrait {
    /// Returns the value of the path at `t` (0.0 to 1.0).
    fn evaluate(&self, t: f64) -> Complex<f64>;
}

#[enum_dispatch(PathTrait)]
#[derive(Debug)]
/// Enum to represent all paths.
pub enum Path {
    PointPath,
    SegmentPath,
    SVGPath,
}
