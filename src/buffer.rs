//! Traits to leverage different input data properties
mod sliceimpls;
mod strimpls;

/// [Buffer] values have a len of unspecified units and can be empty
///
/// Implementors of [ParserCore::feed](crate::parser::ParserCore::feed) should be aware to handle empty buffers.
pub trait Buffer: PartialEq + ToOwned {
    /// Whether the buffer contains no items
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// The number of items in the buffer
    fn len(&self) -> usize;
}

/// A [SplitBuffer] can be split into a prefix and suffix at an offset.
pub trait SplitBuffer: Buffer {
    /// Split the buffer at the given index which must be `<= self.len()`
    fn split_at(&self, mid: usize) -> (&Self, &Self);

    /// Drop the first `mid` items, where `mid` must be `<= self.len()`
    fn drop_up_to(&self, mid: usize) -> &Self {
        let (_, suffix) = self.split_at(mid);
        suffix
    }
}
