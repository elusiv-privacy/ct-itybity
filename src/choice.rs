use crate::{BitIterable, BitLength, BitOrder, GetBit};
use subtle::Choice;

impl BitLength for Choice {
    const BITS: usize = 1;
}

impl<O> GetBit<O> for Choice
where
    O: BitOrder,
{
    fn get_bit(&self, index: usize) -> Choice {
        assert!(index < 1, "index out of bounds");
        *self
    }
}

impl BitIterable for Choice {}
