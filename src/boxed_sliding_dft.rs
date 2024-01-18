use std::{f64::consts::TAU, ops::MulAssign};

use array_math::ArrayOps;
use num::{complex::ComplexFloat, Complex, Float, Zero};
use crate::{FFT, FFTUsingAlgorithm};

pub struct BoxedSlidingDFT<F, const N: usize>{
    x_f: Box<[Complex<F>; N]>,
    x: Box<[F; N]>
}

impl<F, const N: usize> Default for BoxedSlidingDFT<F, N>
where
    Complex<F>: ComplexFloat,
    F: Float
{
    fn default() -> Self
    {
        Self {
            x_f: ArrayOps::fill_boxed(|_| Complex::zero()),
            x: ArrayOps::fill_boxed(|_| F::zero())
        }
    }
}

impl<F, const N: usize> BoxedSlidingDFT<F, N>
where
    Complex<F>: ComplexFloat + MulAssign,
    F: Float
{
    pub fn new<'a, A>(x: Box<[F; N]>) -> Self
    where
        Vec<F>: FFTUsingAlgorithm<Vec<Complex<F>>, A>
    {
        let mut x_f = x.to_vec().fft::<A>().into_iter();
        let x_f = ArrayOps::fill_boxed(|_| x_f.next().unwrap());

        Self {
            x_f,
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