use crate::buffer::{BufRef, Buffer};

// TODO: generalize to Vec<T>
impl Buffer for Vec<u8> {}

impl<T> BufRef for [T]
where
    [T]: ToOwned,
    T: PartialEq,
{
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    fn split_at(&self, mid: usize) -> (&Self, &Self) {
        <[T]>::split_at(self, mid)
    }
}
