use crate::buffer::BufRef;
use crate::parser::ParserCore;
use crate::sequence::{Collect, Foldl};

/// A sequence parser produces an item and a continuation state for multiple items in a sequence
pub trait SequenceParser<B>: ParserCore<B, Output = Option<(Self, Self::Item)>>
where
    B: ?Sized + BufRef,
{
    /// A [SequenceParser] parses zero or more items
    type Item;

    /// Fold each parsed item into an `A` accumulator
    fn foldl<F, A>(self, acc: A, f: F) -> Foldl<Self, F, A, B>
    where
        F: Fn(A, Self::Item) -> A,
    {
        Foldl::new(self, acc, f)
    }

    /// Collect each parsed item into container `C`
    fn collect<C>(self) -> Collect<Self, C>
    where
        C: Default + Extend<Self::Item>,
    {
        Collect::<_, C>::from(self)
    }
}

impl<B, P, X> SequenceParser<B> for P
where
    B: ?Sized + BufRef,
    P: ParserCore<B, Output = Option<(Self, X)>>,
{
    type Item = X;
}
