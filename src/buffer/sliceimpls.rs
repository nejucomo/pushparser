use crate::buffer::{BacktrackBuffer, Buffer, SplitBuffer};

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

impl<T> BacktrackBuffer for [T]
where
    [T]: ToOwned<Owned = Vec<T>>,
    T: PartialEq + Clone,
{
    fn push_onto(&self, backtrack: &mut Vec<T>) {
        backtrack.extend_from_slice(self)
    }
}
