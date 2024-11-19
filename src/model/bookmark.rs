use crate::{database, field};

/// Bookmark by user
#[derive(Debug, Default, Clone)]
pub struct Bookmark {
    pub id: field::Id,
    pub title: String,
    pub url: String,
    pub description: String,
    pub user_id: String,
    pub created_at: field::DateTime,
    pub updated_at: field::DateTime,
}

impl TryFrom<database::Row> for Bookmark {
    type Error = database::Error;

    fn try_from(row: database::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get("id").unwrap().value().clone().try_into()?,
            title: row.get("title").unwrap().value().clone().try_into()?,
            url: row.get("url").unwrap().value().clone().try_into()?,
            description: row.get("description").unwrap().value().clone().try_into()?,
            user_id: row.get("user_id").unwrap().value().clone().try_into()?,
            created_at: row.get("created_at").unwrap().value().clone().try_into()?,
            updated_at: row.get("updated_at").unwrap().value().clone().try_into()?,
        })
    }
}

impl TryFrom<Bookmark> for database::Row {
    type Error = database::Error;

    fn try_from(value: Bookmark) -> Result<Self, Self::Error> {
        let Bookmark {
            id,
            title,
            url,
            description,
            user_id,
            created_at,
            updated_at,
        } = value;
        Ok(vec![
            database::Column::from_pair("id", id),
            database::Column::from_pair("title", title),
            database::Column::from_pair("url", url),
            database::Column::from_pair("description", description),
            database::Column::from_pair("user_id", user_id),
            database::Column::from_pair("created_at", created_at),
            database::Column::from_pair("updated_at", updated_at),
        ]
        .into())
    }
}
