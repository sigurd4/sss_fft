use std::marker::PhantomData;

use num::Zero;

use crate::FFTParalellism;

use super::*;

pub struct FFTAlgorithmRecursive<Paralellism>(PhantomData<Paralellism>);


impl<F, Input, P> FFTAlgorithm<Input, Vec<Complex<F>>> for FFTAlgorithmRecursive<P>
where
    F: Float + Zero,
    Input: IntoIterator<Item: Into<Complex<F>>, IntoIter: IterSpread> + Len,
    P: FFTParalellism<Input, Vec<Complex<F>>>
{
    fn transform<const DIR: bool, Map>(x: Input, map: &Map) -> Vec<Complex<F>>
    where
        Map: Fn(<<Input as IntoIterator>::IntoIter as IterSpread>::Output) -> Vec<Complex<F>> + Send + Sync
    {
        let len = x.len();
        if len <= 1
        {
            return x.into_iter().map(|x| x.into()).collect()
        }

        assert_eq!(len%2, 0, "Length is not divisible by two");

        let [y_even, y_odd] = x.into_iter()
            .spread_array()
            .map(map);

        transform_recursive::<DIR, _, _, _>(
            |y| y,
            y_even,
            y_odd,
            vec![Complex::zero(); len],
            len
        )
    }
}

impl<F, X, const LENGTH: usize, P> FFTAlgorithm<[X; LENGTH], [Complex<F>; LENGTH]> for FFTAlgorithmRecursive<P>
where
    F: Float + Zero,
    X: Into<Complex<F>>,
    [(); (LENGTH + 1) % 2 - 1]:
{
    fn transform<const DIR: bool, Map>(x: [X; LENGTH], map: &Map) -> [Complex<F>; LENGTH]
    where
        Map: Fn(<<[X; LENGTH] as IntoIterator>::IntoIter as IterSpread>::Output) -> Vec<Complex<F>> + Send + Sync
    {
        if LENGTH <= 1
        {
            return x.map(|x| x.into())
        }

        //let [y_even, y_odd] = x.pad().map(map);
        let [y_even, y_odd] = x.into_iter()
        .spread_array()
        .map(map);

        transform_recursive::<DIR, _, _, _>(
            |y| y,
            y_even,
            y_odd,
            [Complex::zero(); LENGTH],
            LENGTH
        )
    }
}