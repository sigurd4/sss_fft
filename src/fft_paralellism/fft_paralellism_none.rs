use crate::FFTParalellism;
use iter_spread::IterSpread;

pub struct FFTParalellismNone;

impl<Input, Output> FFTParalellism<Input, Output> for FFTParalellismNone
where
    Input: IntoIterator<IntoIter: IterSpread>,
    Output: IntoIterator
{
    fn divide_and_conquer<Map>(
        map: &Map,
        x_even_odd: [<<Input as IntoIterator>::IntoIter as IterSpread>::Output; 2]
    ) -> [Vec<Output::Item>; 2]
    where
        Map: Fn(<<Input as IntoIterator>::IntoIter as IterSpread>::Output) -> Vec<Output::Item> + ?Sized
    {
        x_even_odd.map(map)
    }
}