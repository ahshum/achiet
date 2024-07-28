use crate::database::Row;
use chrono::{offset::Utc, DateTime};

pub const TAG_TABLE: &str = "tag";

#[derive(Debug, Default, Clone)]
pub struct Tag {
    pub id: String,
    pub path: String,
    pub prefix: String,
    pub name: String,
    pub label: Option<String>,
    pub parent_id: Option<String>,
    pub depth: u32,
    pub value_type: Option<String>,
    pub user_id: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Tag {
    pub fn from_path(path: String) -> Self {
        let formatted = path.trim_start_matches("/").trim_end_matches("/");
        let mut splits = formatted.split("/").collect::<Vec<_>>();
        let depth = splits.len();
        let name = splits.pop().unwrap_or(formatted);
        let prefix = splits.join("/");

        Self {
            id: String::new(),
            path: "/".to_string() + &formatted,
            prefix: "/".to_string() + &prefix,
            name: name.to_string(),
            depth: u32::try_from(depth).unwrap(),
            ..Default::default()
        }
    }
}

impl TryFrom<Row> for Tag {
    type Error = ();

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id".into()).unwrap().try_into().unwrap(),
            path: row.try_get("path".into()).unwrap().try_into().unwrap(),
            prefix: row.try_get("prefix".into()).unwrap().try_into().unwrap(),
            name: row.try_get("name".into()).unwrap().try_into().unwrap(),
            label: row.try_get("label".into()).unwrap().try_into().unwrap(),
            parent_id: row.try_get("parent_id".into()).unwrap().try_into().unwrap(),
            depth: row.try_get("depth".into()).unwrap().try_into().unwrap(),
            value_type: row
                .try_get("value_type".into())
                .unwrap()
                .try_into()
                .unwrap(),
            user_id: row.try_get("user_id".into()).unwrap().try_into().unwrap(),
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

#[derive(Debug, Clone)]
pub enum TaggedType {
    Bookmark,
}

impl TaggedType {
    pub fn table<'a>(&self) -> &'a str {
        match self {
            TaggedType::Bookmark => "tagged_bookmark",
            _ => "",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct TaggedItem {
    pub id: String,
    pub ref_id: String,
    pub tag_id: String,
    pub value: Option<String>,
}

impl TryFrom<Row> for TaggedItem {
    type Error = ();

    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.try_get("id".into()).unwrap().try_into().unwrap(),
            ref_id: row.try_get("ref_id".into()).unwrap().try_into().unwrap(),
            tag_id: row.try_get("tag_id".into()).unwrap().try_into().unwrap(),
            value: row.try_get("value".into()).unwrap().try_into().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_from_path() {
        assert_eq!(
            Tag::from_path("tag".to_string()),
            Tag {
                path: "/tag".to_string(),
                prefix: "/".to_string(),
                name: "tag".to_string(),
                depth: 1u32,
                ..Tag::default()
            },
        );

        assert_eq!(
            Tag::from_path("/top/subpath/tag/".to_string()),
            Tag {
                path: "/top/subpath/tag".to_string(),
                prefix: "/top/subpath".to_string(),
                name: "tag".to_string(),
                depth: 3u32,
                ..Tag::default()
            },
        );
    }
}
