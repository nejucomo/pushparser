pub enum Update<S, O> {
    /// All input was consumed, but the parser has not completed a full item
    Pending(S),

    /// The parser successfully parsed an item, and consumed the given number of tokens
    Parsed(O, usize),
}
