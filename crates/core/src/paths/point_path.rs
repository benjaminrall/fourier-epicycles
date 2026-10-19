use num::Complex;
use crate::paths::PathTrait;

/// Path representing a stationary point in the complex plane.
#[derive(Debug)]
pub struct PointPath {
    point: Complex<f64>,
}

impl PointPath {
    pub fn new(point: Complex<f64>) -> PointPath {
        PointPath { point }
    }
}

impl PathTrait for PointPath {
    fn evaluate(&self, t: f64) -> Complex<f64> {
        self.point
    }
} 