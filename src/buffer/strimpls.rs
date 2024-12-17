use crate::buffer::{Buffer, SplitBuffer};

impl Buffer for str {
    fn len(&self) -> usize {
        <str>::len(self)
    }
}

impl SplitBuffer for str {
    fn split_at(&self, mid: usize) -> (&Self, &Self) {
        <str>::split_at(self, mid)
    }
}
