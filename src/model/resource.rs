use chrono::{offset::Utc, DateTime};

#[derive(Debug, Default, Clone)]
pub struct Resource {
    pub id: String,
    pub url: String,
    pub protocol: String,
    pub host: String,
    pub path: Option<String>,
    pub query: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
