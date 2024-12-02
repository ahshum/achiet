use super::Error;
use crate::field;

pub fn new_id() -> String {
    ulid::Ulid::new().to_string()
}

pub fn now() -> field::DateTime {
    chrono::Utc::now()
}

pub fn expect_single_row<T>(res: T) -> impl FnOnce(usize) -> Result<T, Error> {
    move |count: usize| {
        if count == 1 {
            Ok(res)
        } else {
            Err(Error::AffectedRows { exp: 1, got: count })
        }
    }
}
