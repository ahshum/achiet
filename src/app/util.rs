use chrono::{offset::Utc, DateTime};
use ulid::Ulid;

pub fn new_uid() -> String {
    Ulid::new().to_string()
}

pub fn now() -> DateTime<Utc> {
    Utc::now()
}
