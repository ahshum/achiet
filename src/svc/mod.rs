mod bookmark;
mod tag;
mod tagging;
mod user;

pub use bookmark::*;
pub use tag::*;
pub use tagging::*;
pub use user::*;

#[derive(Debug)]
pub enum Error {
    NotFound,
    Repo(crate::repo::Error),
    PasswordHash(argon2::password_hash::Error),
    Jwt,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::NotFound => {
                write!(f, "not found")
            }
            Self::Repo(e) => e.fmt(f),
            Self::PasswordHash(e) => e.fmt(f),
            Self::Jwt => write!(f, "jwt encode or decode"),
        }
    }
}
