use array_par_map::ArrayParMap;
use iter_spread::IterSpread;

use crate::FFTParalellism;

pub struct FFTParalellismPar;

impl<Input, Output> FFTParalellism<Input, Output> for FFTParalellismPar
where
    Input: IntoIterator<IntoIter: IterSpread<Output: Send>>,
    Output: IntoIterator<Item: Send>
{
    fn divide_and_conquer<Map>(
        map: &Map,
        x_even_odd: [<<Input as IntoIterator>::IntoIter as IterSpread>::Output; 2]
    ) -> [Vec<Output::Item>; 2]
    where
        Map: Fn(<<Input as IntoIterator>::IntoIter as IterSpread>::Output) -> Vec<Output::Item> + Send + Sync
    {
        x_even_odd.par_map(map)
    }
}