use crate::database::{
    db::Connection,
    query::{Column, ColumnIndex, Query, Row},
    value::{Value, Values},
};

impl From<sqlx::sqlite::SqliteRow> for Row {
    fn from(row: sqlx::sqlite::SqliteRow) -> Self {
        use sqlx::{Column as SqlxColumn, Row, TypeInfo};

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
                    Value::Int(row.try_get(c.ordinal()).unwrap()),
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
        Self { columns }
    }
}

impl<'q> sqlx::IntoArguments<'q, sqlx::sqlite::Sqlite> for Values {
    fn into_arguments(self) -> sqlx::sqlite::SqliteArguments<'q> {
        use sqlx::Arguments;
        let mut args = sqlx::sqlite::SqliteArguments::default();
        for value in self.0.into_iter() {
            match value {
                Value::Bool(b) => args.add(b),
                Value::String(s) => args.add(s),
                Value::DateTime(dt) => args.add(dt),
                Value::Int(i) => args.add(i),
                Value::Unsigned(u) => args.add(i32::try_from(u.unwrap()).unwrap()),
                Value::Bytes(b) => args.add(b),
                _ => todo!(),
            }
        }
        args
    }
}

impl sqlx::ColumnIndex<sqlx::sqlite::SqliteRow> for ColumnIndex {
    fn index(&self, row: &sqlx::sqlite::SqliteRow) -> Result<usize, sqlx::Error> {
        match self {
            ColumnIndex::Str(name) => name.as_str().index(row),
            ColumnIndex::Int(index) => index.index(row),
        }
    }
}

#[derive(Clone)]
pub struct SqliteConnection {
    pool: sqlx::sqlite::SqlitePool,
}

impl SqliteConnection {
    pub async fn connect(dsn: String) -> Result<Self, ()> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(dsn.as_str())
            .await
            .unwrap();
        Ok(Self { pool })
    }
}

impl Connection for SqliteConnection {
    async fn fetch(&self, query: Query) -> Result<Vec<Row>, ()> {
        let (sql, args) = query.build();
        match sqlx::query_with(sql.as_str(), args)
            .fetch_all(&self.pool)
            .await
        {
            Ok(result) => Ok(result.into_iter().map(|row| Row::from(row)).collect()),
            Err(_) => Err(()),
        }
    }

    async fn fetch_one(&self, query: Query) -> Result<Row, ()> {
        let (sql, args) = query.build();
        match sqlx::query_with(sql.as_str(), args)
            .fetch_one(&self.pool)
            .await
        {
            Ok(result) => Ok(Row::from(result)),
            Err(_) => Err(()),
        }
    }

    async fn execute(&self, query: Query) -> Result<(), String> {
        let (sql, args) = query.build();
        match sqlx::query_with(sql.as_str(), args)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
