use std::{ops::{Mul, Deref}, process::Output};

use len_trait::Len;
use num::{Float, Zero, complex::{Complex, ComplexFloat}, NumCast};
use crate::{FFTAlgorithm, FFTAlgorithmDefault, FFTAlgorithmRecursive, FFTParalellismNone, FFTParalellismPar};

pub trait IFFT<Output>
{
    fn ifft<A>(&self) -> Output
    where
        Self: IFFTUsingAlgorithm<Output, A>;
    fn ifft_rec(&self) -> Output
    where
        Self: IFFTUsingAlgorithm<Output, FFTAlgorithmRecursive<FFTParalellismNone>>
    {
        self.ifft::<FFTAlgorithmRecursive<FFTParalellismNone>>()
    }
    fn ifft_rec_par(&self) -> Output
    where
        Self: IFFTUsingAlgorithm<Output, FFTAlgorithmRecursive<FFTParalellismPar>>
    {
        self.ifft::<FFTAlgorithmRecursive<FFTParalellismPar>>()
    }
}
pub trait IntoIFFT<Output>
{
    fn into_ifft<A>(self) -> Output
    where
        Self: IntoIFFTUsingAlgorithm<Output, A>;
    fn into_ifft_rec(self) -> Output
    where
        Self: IntoIFFTUsingAlgorithm<Output, FFTAlgorithmRecursive<FFTParalellismNone>>
    {
        self.into_ifft::<FFTAlgorithmRecursive<FFTParalellismNone>>()
    }
    fn into_ifft_rec_par(self) -> Output
    where
        Self: IntoIFFTUsingAlgorithm<Output, FFTAlgorithmRecursive<FFTParalellismPar>>
    {
        self.into_ifft::<FFTAlgorithmRecursive<FFTParalellismPar>>()
    }
}

impl<Input, Output> IFFT<Output> for Input
where
    Input: IntoIterator<Item = Output::Item>,
    Output: IntoIterator<Item: ComplexFloat>
{
    fn ifft<A>(&self) -> Output
    where
        Self: IFFTUsingAlgorithm<Output, A>
    {
        self.ifft_use_alg()
    }
}
impl<Input, Output> IntoIFFT<Output> for Input
where
    for<'a> &'a Input: IntoIterator<Item: Deref<Target = Output::Item> + 'a>,
    Output: IntoIterator<Item: ComplexFloat>
{
    fn into_ifft<A>(self) -> Output
    where
        Self: IntoIFFTUsingAlgorithm<Output, A>
    {
        self.into_ifft_use_alg()
    }
}

pub trait IFFTUsingAlgorithm<Output, A = FFTAlgorithmDefault>
{
    fn ifft_use_alg(&self) -> Output;
}
pub trait IntoIFFTUsingAlgorithm<Output, A = FFTAlgorithmDefault>: Sized
{
    fn into_ifft_use_alg(self) -> Output;
    #[doc(hidden)]
    fn into_ifft_no_scale_use_alg(self) -> Output;
}

impl<F: Float, V, A> IFFTUsingAlgorithm<Vec<Complex<F>>, A> for V
where
    V: ?Sized,
    for<'a> &'a V: IntoIterator<Item = &'a Complex<F>>,
    Vec<Complex<F>>: IntoIFFTUsingAlgorithm<Vec<Complex<F>>, A>
{
    fn ifft_use_alg(&self) -> Vec<Complex<F>>
    {
        self.into_iter()
            .map(|x| *x)
            .collect::<Vec<Complex<F>>>()
            .into_ifft_use_alg()
    }
}

impl<F: Float, const LENGTH: usize, V, A> IFFTUsingAlgorithm<[Complex<F>; LENGTH], A> for V
where
    V: IntoIFFTUsingAlgorithm<[Complex<F>; LENGTH], A> + Copy + ?Sized
{
    fn ifft_use_alg(&self) -> [Complex<F>; LENGTH]
    {
        (*self).into_ifft_use_alg()
    }
}
impl<Input, C, A> IntoIFFTUsingAlgorithm<Vec<C>, A> for Input
where
    C: ComplexFloat + Mul<C::Real, Output = C>,
    A: FFTAlgorithm<Input, Vec<C>> + FFTAlgorithm<Vec<Input::Item>, Vec<C>>,
    Input: IntoIterator<Item: Into<C> + Sized> + Len
{
    fn into_ifft_use_alg(self) -> Vec<C>
    {
        let q: C::Real = Float::recip(NumCast::from(self.len()).unwrap());
        IntoIFFTUsingAlgorithm::<Vec<C>, A>::into_ifft_no_scale_use_alg(self)
            .into_iter()
            .map(|yn| yn*q)
            .collect()
    }
    fn into_ifft_no_scale_use_alg(self) -> Vec<C>
    {
        A::transform::<false, _>(self, &IntoIFFTUsingAlgorithm::<Vec<C>, A>::into_ifft_no_scale_use_alg)
    }
}
impl<Input, C, const LENGTH: usize, A> IntoIFFTUsingAlgorithm<[C; LENGTH], A> for Input
where
    C: ComplexFloat + Mul<C::Real, Output = C>,
    A: FFTAlgorithm<Input, [C; LENGTH]> + FFTAlgorithm<Vec<Input::Item>, Vec<C>>,
    Input: IntoIterator<Item: Into<C> + Sized>
{
    fn into_ifft_use_alg(self) -> [C; LENGTH]
    {
        let q: C::Real = Float::recip(NumCast::from(LENGTH).unwrap());
        IntoIFFTUsingAlgorithm::<[C; LENGTH], A>::into_ifft_no_scale_use_alg(self)
            .map(|yn| yn*q)
    }
    fn into_ifft_no_scale_use_alg(self) -> [C; LENGTH]
    {
        A::transform::<false, _>(self, &IntoIFFTUsingAlgorithm::<Vec<C>, A>::into_ifft_no_scale_use_alg)
    }
}