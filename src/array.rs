use crate::FromBitIterator;
use subtle::Choice;

impl<const N: usize, T> FromBitIterator for [T; N]
where
    T: FromBitIterator,
{
    fn from_lsb0_iter(iter: impl IntoIterator<Item = Choice>) -> Self {
        let mut iter = iter.into_iter();
        core::array::from_fn(|_| T::from_lsb0_iter(iter.by_ref()))
    }

    fn from_msb0_iter(iter: impl IntoIterator<Item = Choice>) -> Self {
        let mut iter = iter.into_iter();
        core::array::from_fn(|_| T::from_msb0_iter(iter.by_ref()))
    }
}
