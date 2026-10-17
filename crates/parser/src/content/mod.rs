//! ID3v2 Frame Content

#[allow(clippy::module_inception, reason = "private module re-exported below")]
mod content;
mod frames;

pub use self::content::Content;
pub use self::frames::*;
