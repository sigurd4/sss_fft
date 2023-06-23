mod fft_algorithm_recursive; pub use fft_algorithm_recursive::*;
use len_trait::Len;

use std::f64::consts::TAU;

use iter_spread::IterSpread;
use num::{Float, Complex, One};

use crate::FFTParalellismNone;

pub type FFTAlgorithmDefault = FFTAlgorithmRecursive<FFTParalellismNone>;
pub trait FFTAlgorithm<Input, Output>
where
    Output: IntoIterator,
    Input: IntoIterator<Item: Into<Output::Item>, IntoIter: IterSpread>,
{
    fn transform<const DIR: bool, Map>(x: Input, map: &Map) -> Output
    where
        Map: Fn(<<Input as IntoIterator>::IntoIter as IterSpread>::Output) -> Vec<Output::Item> + Send + Sync;
}

fn transform_recursive<'a, const DIR: bool, F, Input, Output>(
    as_slice_mut: impl Fn(&mut Output) -> &mut [Complex<F>],
    y_even: Input,
    y_odd: Input,
    mut y: Output,
    len: usize
) -> Output
where
    F: Float + One + 'a,
    Input: IntoIterator<Item = Complex<F>, IntoIter: IterSpread>
{
    //debug_assert_eq!(len, y_even.length() + y_odd.length());

    let q: F = F::from(len).unwrap().recip();

    let z_rot = Complex::cis(F::from(TAU).unwrap()*if DIR {q} else {-q});
    let mut z: Complex<F> = Complex::one();

    y_even.into_iter()
        .zip(y_odd.into_iter()
            .map(|o| {
                let o = z*o;
                z = z*z_rot;
                o
            })
        )
        .map(|(e, o)| (e + o, e - o))
        .zip({
            let len_odd = len/2;
            let (yp, ym) = as_slice_mut(&mut y).split_at_mut(len_odd);
            yp.into_iter().zip(ym.into_iter())
        })
        .for_each(|((p, m), (yp, ym))| (*yp, *ym) = (p, m));
    y
}