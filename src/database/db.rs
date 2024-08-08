use super::{
    query::{Query, Row},
    sqlite::SqliteConnection,
};

#[derive(Clone)]
pub enum Database {
    Sqlite(SqliteConnection),
    MySql,
    Postgres,
}

impl Database {
    pub fn connection(&self) -> impl Connection {
        match self {
            Self::Sqlite(conn) => (*conn).clone(),
            _ => todo!(),
        }
    }
}

#[trait_variant::make(Connection: Send)]
pub trait LocalConnection {
    async fn fetch(&self, query: Query) -> Result<Vec<Row>, ()>;
    async fn fetch_one(&self, query: Query) -> Result<Row, ()>;
    async fn execute(&self, query: Query) -> Result<(), String>;
}

pub async fn connect(dsn: String) -> Result<Database, ()> {
    match dsn {
        ref s if s.starts_with("sqlite:") => Ok(Database::Sqlite(
            SqliteConnection::connect(dsn).await.unwrap(),
        )),
        _ => todo!(),
    }
}
