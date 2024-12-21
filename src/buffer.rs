//! [Buffer] and other traits to leverage different input data properties
mod manager;
mod sliceimpls;
mod strimpls;

pub use self::manager::BufferManager;

/// A [Buffer] provides storage for input to place pre-parsed data
pub trait Buffer: AsRef<[u8]> + AsMut<[u8]> {}

/// [BufRef] values have a len of unspecified units and can be empty
///
/// Implementors of [PushParser::feed](crate::parser::PushParser::feed) should be aware to handle empty buffers.
pub trait BufRef: PartialEq {
    /// Whether the buffer contains no items
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The number of items in the buffer
    fn len(&self) -> usize;

    /// Split the buffer at the given index which must be `<= self.len()`
    fn split_at(&self, mid: usize) -> (&Self, &Self);

    /// Drop the first `mid` items, where `mid` must be `<= self.len()`
    fn drop_up_to(&self, mid: usize) -> &Self {
        let (_, suffix) = self.split_at(mid);
        suffix
    }
}
