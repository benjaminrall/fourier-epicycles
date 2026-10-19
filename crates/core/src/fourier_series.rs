use std::f64::consts::PI;
use num::Complex;
use crate::paths::{Path, PathTrait};

/// Represents a Discrete Fourier Transform of a complex path,
/// intended for rendering epicycles.
pub struct FourierSeries {
    path: Path,
    coefficients: Vec<Complex<f64>>,
}

/// Inline function to map vector indices to frequencies
#[inline]
fn to_freq(i: usize) -> isize {
    if i % 2 == 0 {
        -((i / 2) as isize)
    } else {
        ((i + 1) / 2) as isize
    }
}

impl FourierSeries {
    /// Initialises a Fourier series for the given path.
    pub fn new(path: Path) -> FourierSeries {
        FourierSeries {
            path,
            coefficients: Vec::new(),
        }
    }

    /// Calculates the constant c_n for some frequency n using numerical integration.
    fn calculate_constant(&self, n: isize, samples: usize) -> Complex<f64> {
        let mut c = Complex::new(0.0, 0.0);

        let k = -n as f64 * 2.0 * PI * Complex::i();
        let dt = 1.0 / samples as f64;

        for i in 0..samples {
            let t = i as f64 * dt;
            c += self.path.evaluate(t) * (k * t).exp() * dt;
        }

        c
    }

    /// Compiles the Fourier series for a specified number of frequencies.
    pub fn compile(&mut self, frequencies: usize, samples: usize) {
        let len_coefficients = 2 * frequencies + 1;
        let mut coefficients = Vec::with_capacity(len_coefficients);

        for i in 0..len_coefficients {
            let n = to_freq(i);
            coefficients.push(self.calculate_constant(n, samples));
        }

        self.coefficients = coefficients;
    }

    /// Computes the terms of each frequency in the Fourier series at `t`.
    pub fn compute_terms(&self, t: f64) -> Vec<Complex<f64>> {
        let mut terms = Vec::with_capacity(self.coefficients.len());

        let k = t * 2.0 * PI * Complex::i();

        for (i, c) in self.coefficients.iter().enumerate() {
            let n = to_freq(i);
            terms.push(c * (k * n as f64).exp());
        }

        terms
    }

    /// Computes the partial cumulative sums of the Fourier series at time `t`.
    pub fn compute_partial_sums(&self, t: f64) -> Vec<Complex<f64>> {
        let mut current_sum = Complex::new(0.0, 0.0);

        let mut sums = Vec::with_capacity(self.coefficients.len() + 1);
        sums.push(current_sum);

        let k = t * 2.0 * PI * Complex::i();

        for (i, c) in self.coefficients.iter().enumerate() {
            let n = to_freq(i);
            current_sum += c * (k * n as f64).exp();

            sums.push(current_sum);
        }

        sums
    }

    /// Evaluates the sum of the Fourier series at `t`.
    pub fn evaluate(&self, t: f64) -> Complex<f64> {
        let mut result = Complex::new(0.0, 0.0);

        let k = t * 2.0 * PI * Complex::i();

        for (i, c) in self.coefficients.iter().enumerate() {
            let n = to_freq(i);
            result += c * (k * n as f64).exp();
        }

        result
    }
}