use num::Complex;
use crate::paths::PathTrait;

/// Path parsed from an SVG file.
#[derive(Debug)]
pub struct SVGPath {

}

impl SVGPath {
    pub fn new() -> SVGPath {
        SVGPath { }
    }
}

impl PathTrait for SVGPath {
    fn evaluate(&self, t: f64) -> Complex<f64> {
        Complex::new(0.0, 0.0)
    }
}