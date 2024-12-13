use crate::Update;

/// The core parser functionality which must be implemented for new parsers
pub trait ParserCore: Sized {
    type Output;

    fn feed(self, stream: &[u8]) -> Update<Self, Self::Output>;

    fn finalize(self) -> Option<Self::Output>;
}
