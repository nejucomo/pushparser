use crate::{ParserCore, Update};

/// I parse any number of `x` bytes.
pub struct XStarParser(usize);

impl Default for XStarParser {
    fn default() -> Self {
        XStarParser(0)
    }
}

impl ParserCore for XStarParser {
    type Output = usize;

    fn feed(self, mut stream: &[u8]) -> Update<Self, Self::Output> {
        use Update::*;

        let mut cnt = 0;

        while let Some((&b, next_stream)) = stream.split_first() {
            if b == b'x' {
                cnt += 1;
                stream = next_stream;
            } else {
                return Parsed(self.0 + cnt, cnt);
            }
        }

        Pending(XStarParser(self.0 + cnt))
    }

    fn finalize(self) -> Option<Self::Output> {
        Some(self.0)
    }
}
