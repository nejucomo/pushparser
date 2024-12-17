use crate::buffer::{Buffer, SplitBuffer};

impl<T> Buffer for [T]
where
    [T]: ToOwned,
    T: PartialEq,
{
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<T> SplitBuffer for [T]
where
    [T]: ToOwned,
    T: PartialEq,
{
    fn split_at(&self, mid: usize) -> (&Self, &Self) {
        <[T]>::split_at(self, mid)
    }
}
