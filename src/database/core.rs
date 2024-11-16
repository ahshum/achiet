use super::{
    sql::{IntoSql, Row, SqlBuilder},
    Error,
};
use std::{boxed::Box, future::Future, pin::Pin};

/// Basic operations for Db and Tx
pub trait Builder {
    fn sql(&self) -> SqlBuilder;
}

/// Db write operations
pub trait Executor {
    fn execute(
        &mut self,
        sql: impl IntoSql,
    ) -> Pin<Box<dyn Future<Output = Result<usize, Error>> + Send + '_>>;
}

/// Db read operations
pub trait Queryer {
    fn query(
        &mut self,
        sql: impl IntoSql,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Row>, Error>> + Send + '_>>;
}

/// General operations
pub trait Ops: Builder + Executor + Queryer {}

impl<T: Builder + Executor + Queryer> Ops for T {}

/// Tx operations
pub trait TxOps<'a>: Ops {
    fn commit(self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
    fn rollback(self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>>;
}

/// Db operations
pub trait DbOps: Ops {
    fn begin(&self) -> Pin<Box<dyn Future<Output = Result<impl TxOps, Error>> + Send + '_>>;
}
