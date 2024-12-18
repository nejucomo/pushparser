//! Traits for parser implementors and consumers
mod core;
mod intoutf8;
mod read;
mod text;
mod update;

pub use self::core::ParserCore;
pub use self::intoutf8::IntoUtf8Parser;
pub use self::read::ReadParser;
pub use self::text::TextParser;
pub use self::update::Update;
