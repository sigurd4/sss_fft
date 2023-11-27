moddef::moddef!(
    flat(pub) mod {
        fft_paralellism_par,
        fft_paralellism_none
    }
);

use iter_spread::IterSpread;

pub trait FFTParalellism<Input, Output>
where
    Input: IntoIterator<IntoIter: IterSpread>,
    Output: IntoIterator
{
    fn divide_and_conquer<Map>(
        map: &Map,
        x_even_odd: [<<Input as IntoIterator>::IntoIter as IterSpread>::Output; 2]
    ) -> [Vec<Output::Item>; 2]
    where
        Map: Fn(<<Input as IntoIterator>::IntoIter as IterSpread>::Output) -> Vec<Output::Item> + Send + Sync;
}