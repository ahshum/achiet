//! Database engines and utilities

mod conn;
mod core;
mod sql;
mod sqlite;
mod value;

pub use conn::*;
pub use core::*;
pub use sql::*;
pub use sqlite::*;
pub use value::*;

/// General database errors
#[derive(Debug)]
pub enum Error {
    Connection,
    Unsupported,
    ValueParsing,
    Parsing,
    Sqlx(sqlx::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Connection => write!(f, "connection"),
            Self::Unsupported => write!(f, "unsupported"),
            Self::ValueParsing => write!(f, "value parsing"),
            Self::Parsing => write!(f, "parsing"),
            Self::Sqlx(e) => write!(f, "sqlx ").and_then(|_| e.fmt(f)),
        }
    }
}
