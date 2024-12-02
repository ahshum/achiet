use super::{
    core::*,
    sql::{IntoSql, Row, SqlBuilder},
    sqlite::Sqlite,
    Error,
};
use std::{boxed::Box, future::Future, pin::Pin};

/// Connects to the database and returns as `DbOps`
pub async fn connect(url: &str) -> Result<Conn, Error> {
    Conn::connect(url).await
}

/// Connection type
#[derive(Clone, Debug)]
pub enum Conn {
    Sqlite(Sqlite),
}

impl Conn {
    pub async fn connect(url: &str) -> Result<Self, Error> {
        match url {
            ref s if s.starts_with("sqlite:") => Ok(Conn::Sqlite(Sqlite::connect(url).await?)),
            // ref s if s.starts_with("mock:") => Ok(Box::new(Mock{})),
            _ => Err(Error::Unsupported),
        }
    }
}

impl Builder for Conn {
    fn sql(&self) -> SqlBuilder {
        match self {
            Self::Sqlite(d) => d.sql(),
        }
    }
}

impl Executor for Conn {
    fn execute(
        &mut self,
        sql: impl IntoSql,
    ) -> Pin<Box<dyn Future<Output = Result<usize, Error>> + Send + '_>> {
        match self {
            Self::Sqlite(d) => d.execute(sql),
        }
    }
}

impl Queryer for Conn {
    fn query(
        &mut self,
        sql: impl IntoSql,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Row>, Error>> + Send + '_>> {
        match self {
            Self::Sqlite(d) => d.query(sql),
        }
    }
}

impl DbOps for Conn {
    fn begin(&self) -> Pin<Box<dyn Future<Output = Result<impl TxOps, Error>> + Send + '_>> {
        match self {
            Self::Sqlite(d) => d.begin(),
        }
    }
}
