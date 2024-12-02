//! Manages database operations and relationship building

mod bookmark;
mod tag;
mod tagging;
mod user;
mod util;

pub use bookmark::*;
pub use tag::*;
pub use tagging::*;
pub use user::*;

/// Error from either database or identifier
#[derive(Debug)]
pub enum Error {
    AffectedRows { exp: usize, got: usize },
    Database(crate::database::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::AffectedRows { exp, got } => {
                write!(f, "affected rows expected {} got {}", exp, got)
            }
            Self::Database(e) => e.fmt(f),
        }
    }
}
