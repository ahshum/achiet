use crate::{database, field};

/// User account
#[derive(Debug, Default, Clone)]
pub struct User {
    pub id: field::Id,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub role: UserRole,
    pub created_at: field::DateTime,
    pub updated_at: field::DateTime,
}

impl TryFrom<database::Row> for User {
    type Error = database::Error;

    fn try_from(row: database::Row) -> Result<Self, Self::Error> {
        Ok(Self {
            id: row.get("id").unwrap().value().clone().try_into()?,
            username: row.get("username").unwrap().value().clone().try_into()?,
            password: row.get("password").unwrap().value().clone().try_into()?,
            email: row.get("email").unwrap().value().clone().try_into()?,
            role: String::try_from(row.get("role").unwrap().value().clone())?
                .parse()
                .expect("user role parse error"),
            created_at: row.get("created_at").unwrap().value().clone().try_into()?,
            updated_at: row.get("updated_at").unwrap().value().clone().try_into()?,
        })
    }
}

impl TryFrom<User> for database::Row {
    type Error = database::Error;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        let User {
            id,
            username,
            password,
            email,
            role,
            created_at,
            updated_at,
        } = user;
        Ok(vec![
            database::Column::from_pair("id", id),
            database::Column::from_pair("username", username),
            database::Column::from_pair("password", password),
            database::Column::from_pair("email", email),
            database::Column::from_pair("role", role.to_string()),
            database::Column::from_pair("created_at", created_at),
            database::Column::from_pair("updated_at", updated_at),
        ]
        .into())
    }
}

/// User roles
#[derive(Debug, Default, Clone, PartialEq)]
pub enum UserRole {
    #[default]
    User,
    Admin,
}

impl std::str::FromStr for UserRole {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Self::User),
            "admin" => Ok(Self::Admin),
            _ => Err("user role parse error".to_string()),
        }
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Admin => write!(f, "admin"),
        }
    }
}
