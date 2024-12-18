//! Parsers which compose subparsers to express sequencing ([Then]), alternatives (`Or` (not yet implemented)), and other constructs
mod then;

pub use self::then::Then;
