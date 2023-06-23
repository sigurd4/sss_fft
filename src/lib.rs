#![feature(iter_array_chunks)]
#![feature(associated_type_bounds)]
#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]

mod fft; pub use fft::*;
mod ifft; pub use ifft::*;
mod fft_algorithm; pub use fft_algorithm::*;
mod fft_paralellism; pub use fft_paralellism::*;

#[cfg(test)]
mod test
{
    use num::Complex;

    use crate::{FFT, IFFT};

    #[test]
    fn it_works()
    {
        let x = [1.0, 2.0, 3.0, 4.0];
        let y: [Complex<f64>; _] = x.fft_rec_par();
        let x: [Complex<f64>; _] = y.ifft_rec_par();
        println!("{:?}", x)
    }

    const N: usize = 4096;

    #[test]
    fn benchmark_rec()
    {
        let x = [0.0; N];
        let y: [_; _] = x.fft_rec();
        let x: [_; _] = y.ifft_rec();
    }
    
    #[test]
    fn benchmark_rec_par()
    {
        let x = [0.0; N];
        let y: [_; _] = x.fft_rec_par();
        let x: [_; _] = y.ifft_rec_par();
    }
}