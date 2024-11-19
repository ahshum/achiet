use crate::{database, field};

/// Define polymorphic relationship for tagged item
#[derive(Debug, Default, Clone)]
pub struct Tagging {
    pub id: field::Id,
    pub item_id: String,
    pub tag_id: String,
    pub value: Option<String>,
    pub order_index: usize,
}

impl TryFrom<database::Row> for Tagging {
    type Error = database::Error;

    fn try_from(row: database::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get("id").unwrap().value().clone().try_into()?,
            item_id: row.get("item_id").unwrap().value().clone().try_into()?,
            tag_id: row.get("tag_id").unwrap().value().clone().try_into()?,
            value: row.get("value").unwrap().value().clone().try_into()?,
            order_index: row.get("order_index").unwrap().value().clone().try_into()?,
        })
    }
}

impl TryFrom<Tagging> for database::Row {
    type Error = database::Error;

    fn try_from(src: Tagging) -> Result<Self, Self::Error> {
        let Tagging {
            id,
            item_id,
            tag_id,
            value,
            order_index,
        } = src;
        Ok(vec![
            database::Column::from_pair("id", id),
            database::Column::from_pair("item_id", item_id),
            database::Column::from_pair("tag_id", tag_id),
            database::Column::from_pair("value", value),
            database::Column::from_pair("order_index", order_index),
        ]
        .into())
    }
}

/// Tagging types
#[derive(Debug, PartialEq, Clone)]
pub enum TaggingType {
    Bookmark,
}

impl std::fmt::Display for TaggingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bookmark => write!(f, "bookmark"),
        }
    }
}
