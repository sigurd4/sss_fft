#![feature(iter_array_chunks)]
#![feature(associated_type_bounds)]
#![feature(generic_const_exprs)]
#![feature(generic_arg_infer)]

moddef::moddef!(
    flat(pub) mod {
        fft,
        ifft,
        fft_algorithm,
        fft_paralellism,

        sliding_dft,
        boxed_sliding_dft
    }
);

#[cfg(test)]
mod test
{
    use array_math::ArrayOps;
    use num::Complex;

    use crate::{BoxedSlidingDFT, FFTAlgorithmRecursive, FFTParalellismNone, FFT, IFFT};

    #[test]
    fn it_works()
    {
        let mut dft = BoxedSlidingDFT::<f64, 0b100000000000000>::default();

        let y = dft.next(1.0);

        println!("{:?}", y[1])
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