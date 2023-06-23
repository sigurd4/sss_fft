use std::ops::Deref;

use len_trait::Len;
use num::{Float, complex::{Complex, ComplexFloat}};
use crate::{FFTAlgorithm, FFTAlgorithmDefault, FFTAlgorithmRecursive, FFTParalellismNone, FFTParalellismPar};

pub trait FFT<Output>
{
    fn fft<A>(&self) -> Output
    where
        Self: FFTUsingAlgorithm<Output, A>;
    fn fft_rec(&self) -> Output
    where
        Self: FFTUsingAlgorithm<Output, FFTAlgorithmRecursive<FFTParalellismNone>>
    {
        self.fft::<FFTAlgorithmRecursive<FFTParalellismNone>>()
    }
    fn fft_rec_par(&self) -> Output
    where
        Self: FFTUsingAlgorithm<Output, FFTAlgorithmRecursive<FFTParalellismPar>>
    {
        self.fft::<FFTAlgorithmRecursive<FFTParalellismPar>>()
    }
}
pub trait IntoFFT<Output>: Sized
{
    fn into_fft<A>(self) -> Output
    where
        Self: IntoFFTUsingAlgorithm<Output, A>;
    fn into_fft_rec(self) -> Output
    where
        Self: IntoFFTUsingAlgorithm<Output, FFTAlgorithmRecursive<FFTParalellismNone>>
    {
        self.into_fft::<FFTAlgorithmRecursive<FFTParalellismNone>>()
    }
    fn into_fft_rec_par(self) -> Output
    where
        Self: IntoFFTUsingAlgorithm<Output, FFTAlgorithmRecursive<FFTParalellismPar>>
    {
        self.into_fft::<FFTAlgorithmRecursive<FFTParalellismPar>>()
    }
}

impl<Input, Output> FFT<Output> for Input
where
    Input: IntoIterator<Item: Into<Output::Item>>,
    Output: IntoIterator<Item: ComplexFloat>
{
    fn fft<A>(&self) -> Output
    where
        Self: FFTUsingAlgorithm<Output, A>
    {
        self.fft_use_alg()
    }
}
impl<Input, Output> IntoFFT<Output> for Input
where
    for<'a> &'a Input: IntoIterator<Item: Deref<Target: Into<Output::Item>> + 'a>,
    Output: IntoIterator<Item: ComplexFloat>
{
    fn into_fft<A>(self) -> Output
    where
        Self: IntoFFTUsingAlgorithm<Output, A>
    {
        self.into_fft_use_alg()
    }
}

pub trait FFTUsingAlgorithm<Output, A = FFTAlgorithmDefault>
{
    fn fft_use_alg(&self) -> Output;
}
pub trait IntoFFTUsingAlgorithm<Output, A = FFTAlgorithmDefault>: Sized
{
    fn into_fft_use_alg(self) -> Output;
}

impl<F: Float, V, A> FFTUsingAlgorithm<Vec<Complex<F>>, A> for V
where
    V: ?Sized,
    for<'a> &'a V: IntoIterator<Item: Deref<Target: Into<Complex<F>> + Copy>>,
    A: FFTAlgorithm<Vec<Complex<F>>, Vec<Complex<F>>>,
    Vec<Complex<F>>: IntoFFTUsingAlgorithm<Vec<Complex<F>>, A>
{
    fn fft_use_alg(&self) -> Vec<Complex<F>>
    {
        self.into_iter()
            .map(|x| (*x).into())
            .collect::<Vec<Complex<F>>>()
            .into_fft_use_alg()
    }
}
impl<F: Float, const LENGTH: usize, V, A> FFTUsingAlgorithm<[Complex<F>; LENGTH], A> for V
where
    V: IntoFFTUsingAlgorithm<[Complex<F>; LENGTH], A> + Copy + ?Sized
{
    fn fft_use_alg(&self) -> [Complex<F>; LENGTH]
    {
        (*self).into_fft_use_alg()
    }
}
impl<Input, Output, A> IntoFFTUsingAlgorithm<Output, A> for Input
where
    A: FFTAlgorithm<Input, Output> + FFTAlgorithm<Vec<Input::Item>, Vec<Output::Item>>,
    Input: IntoIterator<Item: Into<Output::Item> + Sized>,
    Output: IntoIterator
{
    fn into_fft_use_alg(self) -> Output
    {
        A::transform::<true, _>(self, &<Vec<Input::Item> as IntoFFTUsingAlgorithm::<Vec<Output::Item>, A>>::into_fft_use_alg)
    }
}