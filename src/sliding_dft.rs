use std::{f64::consts::TAU, ops::MulAssign};

use array_math::ArrayOps;
use num::{complex::ComplexFloat, Complex, Float, Zero};
use crate::{FFT, FFTUsingAlgorithm};

pub struct SlidingDFT<F, const N: usize>{
    x_f: [Complex<F>; N],
    x: [F; N]
}

impl<F, const N: usize> Default for SlidingDFT<F, N>
where
    Complex<F>: ComplexFloat,
    F: Float
{
    fn default() -> Self
    {
        Self {
            x_f: ArrayOps::fill(|_| Complex::zero()),
            x: ArrayOps::fill(|_| F::zero())
        }
    }
}

impl<F, const N: usize> SlidingDFT<F, N>
where
    Complex<F>: ComplexFloat + MulAssign,
    F: Float
{
    pub fn new<A>(x: [F; N]) -> Self
    where
        [F; N]: FFTUsingAlgorithm<[Complex<F>; N], A>
    {
        Self {
            x_f: x.fft::<A>(),
            x
        }
    }
    
    pub fn get_input(&self) -> &[F; N]
    {
        &self.x
    }
    pub fn get_output(&self) -> &[Complex<F>; N]
    {
        &self.x_f
    }

    pub fn next(&mut self, x: F) -> &[Complex<F>; N]
    {
        let xn = self.x[N - 1];
        for n in (0..(N - 1)).rev()
        {
            self.x[n + 1] = self.x[n];
        }
        self.x[0] = x;

        if self.x.iter().all(|x| x.is_zero())
        {
            for x_f in self.x_f.iter_mut()
            {
                *x_f = Complex::zero();
            }
        }
        else
        {
            let x_delta = Complex::from(xn - x);
    
            let mut z_n = Complex::from(F::one());
            let z_1 = Complex::cis(F::from(TAU).unwrap()/F::from(N).unwrap());
    
            for x_f in self.x_f.iter_mut()
            {
                *x_f = z_n*(*x_f + x_delta);
    
                z_n *= z_1;
            }
        }

        &self.x_f
    }
}