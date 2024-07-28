use crate::database::Row;
use chrono::{offset::Utc, DateTime};

pub const USER_TABLE: &str = "user";

#[derive(Debug, Default, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub role: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<Row> for User {
    type Error = ();

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id".into()).unwrap().try_into().unwrap(),
            username: row.try_get("username".into()).unwrap().try_into().unwrap(),
            password: row.try_get("password".into()).unwrap().try_into().unwrap(),
            email: row.try_get("email".into()).unwrap().try_into().unwrap(),
            role: row.try_get("role".into()).unwrap().try_into().unwrap(),
            created_at: row
                .try_get("created_at".into())
                .unwrap()
                .try_into()
                .unwrap(),
            updated_at: row
                .try_get("updated_at".into())
                .unwrap()
                .try_into()
                .unwrap(),
        })
    }
}
