use crate::{database, field};

/// Tag
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Tag {
    pub id: field::Id,
    pub path: String,
    pub prefix: String,
    pub name: String,
    pub label: Option<String>,
    pub parent_id: Option<field::Id>,
    pub depth: usize,
    pub value_type: TagValueType,
    pub user_id: String,
    pub created_at: field::DateTime,
    pub updated_at: field::DateTime,
}

impl Tag {
    /// Parses tag from path
    pub fn from_path(path: impl Into<String>) -> Self {
        let path_str = path.into();
        let formatted = path_str.trim_matches('/');
        let mut splits = formatted.split('/').collect::<Vec<_>>();
        let depth = splits.len();
        let name = splits.pop().unwrap_or(formatted);

        Self {
            path: format!("/{}", formatted),
            prefix: format!("/{}", splits.join("/")),
            name: name.to_string(),
            depth,
            ..Default::default()
        }
    }
}

impl TryFrom<database::Row> for Tag {
    type Error = database::Error;

    fn try_from(row: database::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get("id").unwrap().value().clone().try_into()?,
            path: row.get("path").unwrap().value().clone().try_into()?,
            prefix: row.get("prefix").unwrap().value().clone().try_into()?,
            name: row.get("name").unwrap().value().clone().try_into()?,
            label: row.get("label").unwrap().value().clone().try_into()?,
            parent_id: row.get("parent_id").unwrap().value().clone().try_into()?,
            depth: row.get("depth").unwrap().value().clone().try_into()?,
            value_type: String::try_from(row.get("value_type").unwrap().value().clone())?
                .parse()
                .expect("tag value type parse error"),
            user_id: row.get("user_id").unwrap().value().clone().try_into()?,
            created_at: row.get("created_at").unwrap().value().clone().try_into()?,
            updated_at: row.get("updated_at").unwrap().value().clone().try_into()?,
        })
    }
}

impl TryFrom<Tag> for database::Row {
    type Error = database::Error;

    fn try_from(tag: Tag) -> Result<Self, Self::Error> {
        let Tag {
            id,
            path,
            prefix,
            name,
            label,
            parent_id,
            depth,
            value_type,
            user_id,
            created_at,
            updated_at,
        } = tag;
        Ok(vec![
            database::Column::from_pair("id", id),
            database::Column::from_pair("path", path),
            database::Column::from_pair("prefix", prefix),
            database::Column::from_pair("name", name),
            database::Column::from_pair("label", label),
            database::Column::from_pair("parent_id", parent_id),
            database::Column::from_pair("depth", depth),
            database::Column::from_pair("value_type", value_type.to_string()),
            database::Column::from_pair("user_id", user_id),
            database::Column::from_pair("created_at", created_at),
            database::Column::from_pair("updated_at", updated_at),
        ]
        .into())
    }
}

/// Tag value types
#[derive(Debug, Default, PartialEq, Clone)]
pub enum TagValueType {
    #[default]
    String,
    Number,
}

impl std::str::FromStr for TagValueType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "string" => Ok(Self::String),
            "number" => Ok(Self::Number),
            _ => Err("tag value parse error".to_string()),
        }
    }
}

impl std::fmt::Display for TagValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::Number => write!(f, "number"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tag_from_path() {
        assert_eq!(
            Tag::from_path("tag"),
            Tag {
                path: "/tag".to_string(),
                prefix: "/".to_string(),
                name: "tag".to_string(),
                depth: 1,
                ..Tag::default()
            },
        );

        assert_eq!(
            Tag::from_path("/top/subpath/tag/"),
            Tag {
                path: "/top/subpath/tag".to_string(),
                prefix: "/top/subpath".to_string(),
                name: "tag".to_string(),
                depth: 3,
                ..Tag::default()
            },
        );
    }
}
