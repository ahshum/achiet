use super::{
    core::*,
    sql::{Column, IntoSql, Row, SqlArguments, SqlBuilder},
    value::Value,
    Error,
};
use std::{boxed::Box, future::Future, pin::Pin};

/// Sqlite database
#[derive(Clone, Debug)]
pub struct Sqlite(pub(crate) sqlx::sqlite::SqlitePool);

impl Sqlite {
    pub async fn connect(url: &str) -> Result<Self, Error> {
        sqlx::sqlite::SqlitePool::connect(url)
            .await
            .map_err(|_| Error::Connection)
            .map(Self)
    }
}

impl Builder for Sqlite {
    fn sql(&self) -> SqlBuilder {
        SqlBuilder::new_sqlite()
    }
}

impl Executor for Sqlite {
    fn execute(
        &mut self,
        sql: impl IntoSql,
    ) -> Pin<Box<dyn Future<Output = Result<usize, Error>> + Send + '_>> {
        let (query, args) = sql.into_sql();
        Box::pin(async move {
            sqlx::query_with(&query, args)
                .execute(&self.0)
                .await
                .map_err(Error::Sqlx)
                .and_then(|res| res.rows_affected().try_into().map_err(|_| Error::Parsing))
        })
    }
}

impl Queryer for Sqlite {
    fn query(
        &mut self,
        sql: impl IntoSql,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Row>, Error>> + Send + '_>> {
        let (query, args) = sql.into_sql();
        Box::pin(async move {
            sqlx::query_with(&query, args)
                .fetch_all(&self.0)
                .await
                .map_err(Error::Sqlx)
                .map(|rows| rows.into_iter().map(Row::from).collect())
        })
    }
}

impl DbOps for Sqlite {
    fn begin(&self) -> Pin<Box<dyn Future<Output = Result<impl TxOps, Error>> + Send + '_>> {
        Box::pin(async move { self.0.begin().await.map_err(Error::Sqlx).map(SqliteTx) })
    }
}

/// Sqlite transaction
pub struct SqliteTx<'a>(pub(crate) sqlx::Transaction<'a, sqlx::sqlite::Sqlite>);

impl<'a> Builder for SqliteTx<'a> {
    fn sql(&self) -> SqlBuilder {
        SqlBuilder::new_sqlite()
    }
}

impl<'a> Executor for SqliteTx<'a> {
    fn execute(
        &mut self,
        sql: impl IntoSql,
    ) -> Pin<Box<dyn Future<Output = Result<usize, Error>> + Send + '_>> {
        let (query, args) = sql.into_sql();
        Box::pin(async move {
            sqlx::query_with(&query, args)
                .execute(&mut *self.0)
                .await
                .map_err(Error::Sqlx)
                .and_then(|res| res.rows_affected().try_into().map_err(|_| Error::Parsing))
        })
    }
}

impl<'a> Queryer for SqliteTx<'a> {
    fn query(
        &mut self,
        sql: impl IntoSql,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<Row>, Error>> + Send + '_>> {
        let (query, args) = sql.into_sql();
        Box::pin(async move {
            sqlx::query_with(&query, args)
                .fetch_all(&mut *self.0)
                .await
                .map_err(Error::Sqlx)
                .map(|rows| rows.into_iter().map(Row::from).collect())
        })
    }
}

impl<'a> TxOps<'a> for SqliteTx<'a> {
    fn commit(self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            let Self(inner) = self;
            inner.commit().await.map_err(Error::Sqlx)
        })
    }

    fn rollback(self) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            let Self(inner) = self;
            inner.rollback().await.map_err(Error::Sqlx)
        })
    }
}

/// Row parsing from SqliteRow
///
/// [Type Reference](https://github.com/launchbadge/sqlx/blob/main/sqlx-sqlite/src/types/mod.rs)
/// There is no u64 in Sqlite and the largest positive number is 2^63. So
/// integer from Sqlite is safe to convert into i64.
///
impl From<sqlx::sqlite::SqliteRow> for Row {
    fn from(row: sqlx::sqlite::SqliteRow) -> Self {
        use sqlx::{Column as _, Row as _, TypeInfo as _};

        let columns = row
            .columns()
            .into_iter()
            .map(|c| match c.type_info().name() {
                "TEXT" => Column(
                    c.name().to_string(),
                    Value::String(row.try_get(c.ordinal()).unwrap()),
                ),
                "BOOLEAN" => Column(
                    c.name().to_string(),
                    Value::Bool(row.try_get(c.ordinal()).unwrap()),
                ),
                "INTEGER" => Column(
                    c.name().to_string(),
                    Value::Int64(row.try_get(c.ordinal()).unwrap()),
                ),
                "DATETIME" => Column(
                    c.name().to_string(),
                    Value::DateTime(row.try_get(c.ordinal()).unwrap()),
                ),
                "BLOB" => Column(
                    c.name().to_string(),
                    Value::Bytes(row.try_get(c.ordinal()).unwrap()),
                ),
                _ => todo!(),
            })
            .collect();
        Self(columns)
    }
}

/// SqlArguments parsing into SqliteArguments
impl<'a> sqlx::IntoArguments<'a, sqlx::sqlite::Sqlite> for SqlArguments {
    fn into_arguments(self) -> sqlx::sqlite::SqliteArguments<'a> {
        use sqlx::Arguments as _;

        let mut args = sqlx::sqlite::SqliteArguments::default();
        self.0.into_iter().for_each(|value| {
            let _ = match value {
                Value::Bool(b) => args.add(b),
                Value::Int(i) => args.add(i),
                Value::Int64(i) => {
                    args.add(i.map(|i| i64::try_from(i).expect("sqlite i64 encode")))
                }
                Value::Uint(u) => args.add(u),
                Value::Uint64(u) => {
                    args.add(u.map(|u| i64::try_from(u).expect("sqlite u64 encode")))
                }
                Value::String(s) => args.add(s),
                Value::Bytes(bs) => args.add(bs),
                Value::DateTime(dt) => args.add(dt),
            };
        });
        args
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_sqlite() {
        let mut db = Sqlite::connect("sqlite:test.db").await.unwrap();
        println!("=== db");
        println!(
            "{:?}",
            db.execute(r#"DELETE FROM user WHERE username LIKE 'sqlite-user-%'"#)
                .await
        );
        println!("{:?}", db.execute(r#"INSERT INTO user (id, username, password, role, created_at, updated_at)
            VALUES ('sqlite-user-01', 'sqlite-user-01', 'password', 'role', '2024-10-10 00:00:00', '2024-10-10 00:00:00')"#).await);
        println!("{:?}", db.query(r#"SELECT * FROM user"#).await);

        let mut tx01 = db.begin().await.unwrap();
        println!("=== tx01");
        println!("{:?}", tx01.execute(r#"INSERT INTO user (id, username, password, role, created_at, updated_at)
            VALUES ('sqlite-user-02', 'sqlite-user-02', 'password', 'role', '2024-10-10 00:00:00', '2024-10-10 00:00:00')"#).await);
        println!("{:?}", tx01.rollback().await.unwrap());
        println!("{:?}", db.query(r#"SELECT * FROM user"#).await);

        let mut tx02 = db.begin().await.unwrap();
        println!("=== tx02");
        println!("{:?}", tx02.execute(r#"INSERT INTO user (id, username, password, role, created_at, updated_at)
            VALUES ('sqlite-user-03', 'sqlite-user-03', 'password', 'role', '2024-10-10 00:00:00', '2024-10-10 00:00:00')"#).await);
        println!("{:?}", tx02.commit().await.unwrap());
        println!("{:?}", db.query(r#"SELECT * FROM user"#).await);
    }
}
