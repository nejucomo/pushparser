use crate::parser::Outcome;

/// Provides the number of elements consumed and the outcome
#[derive(Debug)]
pub struct Update<S, X> {
    /// The number of input units consumed
    pub consumed: usize,
    /// The outcome of incremental parsing
    pub outcome: Outcome<S, X>,
}

impl<S, X> Update<S, X> {
    /// Map the outcome
    pub fn map_outcome<F, S2, X2>(self, f: F) -> Update<S2, X2>
    where
        F: FnOnce(Outcome<S, X>) -> Outcome<S2, X2>,
    {
        Update {
            consumed: self.consumed,
            outcome: f(self.outcome),
        }
    }

    /// Map the pending state
    pub fn map_next<F, S2>(self, f: F) -> Update<S2, X>
    where
        F: FnOnce(S) -> S2,
    {
        self.map_outcome(|oc| oc.map_next(f))
    }

    /// Map the output
    pub fn map_output<F, X2>(self, f: F) -> Update<S, X2>
    where
        F: FnOnce(X) -> X2,
    {
        self.map_outcome(|oc| oc.map_output(f))
    }
}
