use crate::buffer::BufRef;

impl BufRef for str {
    fn len(&self) -> usize {
        <str>::len(self)
    }

    fn split_at(&self, mid: usize) -> (&Self, &Self) {
        <str>::split_at(self, mid)
    }
}
