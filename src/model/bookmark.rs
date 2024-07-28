use crate::database::Row;
use chrono::{offset::Utc, DateTime};

pub const BOOKMARK_TABLE: &str = "bookmark";

#[derive(Debug, Default, Clone)]
pub struct Bookmark {
    pub id: String,
    pub user_id: String,
    pub title: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
    pub resource_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<Row> for Bookmark {
    type Error = ();

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id".into()).unwrap().try_into().unwrap(),
            user_id: row.try_get("user_id".into()).unwrap().try_into().unwrap(),
            title: row.try_get("title".into()).unwrap().try_into().unwrap(),
            url: row.try_get("url".into()).unwrap().try_into().unwrap(),
            description: row
                .try_get("description".into())
                .unwrap()
                .try_into()
                .unwrap(),
            resource_id: row
                .try_get("resource_id".into())
                .unwrap()
                .try_into()
                .unwrap(),
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
