use num::Complex;
use crate::paths::PathTrait;

/// Path representing a line segment.
#[derive(Debug)]
pub struct SegmentPath {
    start: Complex<f64>,
    direction: Complex<f64>,
}

impl SegmentPath {
    pub fn new(start: Complex<f64>, end: Complex<f64>) -> SegmentPath {
        let direction = end - start;

        SegmentPath {
            start,
            direction,
        }
    }
}

impl PathTrait for SegmentPath {
    fn evaluate(&self, t: f64) -> Complex<f64> {
        self.start + self.direction * t
    }
} 