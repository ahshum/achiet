use super::value::Value;

/// Wrapper of Vec<[Value]> for supporting [sqlx::IntoArguments] of database
pub struct SqlArguments(pub(crate) Vec<Value>);

/// Builds SQL in a simple way
#[derive(Debug, Default)]
pub struct SqlBuilder {
    sql: String,
    args: Vec<Value>,
    bind_type: SqlBindType,
    bind_count: usize,
}

impl SqlBuilder {
    pub fn new_sqlite() -> Self {
        Self::default()
    }

    pub fn new_mysql() -> Self {
        Self::default()
    }

    pub fn new_postgres() -> Self {
        Self {
            bind_type: SqlBindType::Dollar,
            ..Self::default()
        }
    }

    pub fn sub_sql(&self) -> Self {
        Self::default()
    }

    pub fn push(&mut self, sql: impl Into<String>) -> &mut Self {
        for c in sql.into().chars() {
            match c {
                '?' => {
                    self.bind_count += 1;
                    if let SqlBindType::Dollar = self.bind_type {
                        self.sql.push('$');
                        self.sql.push_str(&self.bind_count.to_string());
                    } else {
                        self.sql.push('?');
                    }
                }
                _ => self.sql.push(c),
            }
        }
        self
    }

    pub fn bind(&mut self, value: impl Into<Value>) -> &mut Self {
        self.args.push(value.into());
        self
    }

    pub fn bind_many(&mut self, values: Vec<impl Into<Value>>) -> &mut Self {
        values.into_iter().for_each(|v| {
            self.bind(v);
        });
        self
    }

    pub fn append(&mut self, sub_sql: impl Into<Self>) -> &mut Self {
        let Self { sql, args, .. } = sub_sql.into();
        self.push(&sql).bind_many(args)
    }
}

/// Convert into SQL query arguments
pub trait IntoSql {
    fn into_sql(self) -> (String, SqlArguments);
}

impl IntoSql for SqlBuilder {
    fn into_sql(self) -> (String, SqlArguments) {
        let Self { sql, args, .. } = self;
        (sql, SqlArguments(args))
    }
}

impl IntoSql for &str {
    fn into_sql(self) -> (String, SqlArguments) {
        (self.to_string(), SqlArguments(Vec::new()))
    }
}

impl IntoSql for String {
    fn into_sql(self) -> (String, SqlArguments) {
        (self, SqlArguments(Vec::new()))
    }
}

/// SQL bind types
#[derive(Debug, Default, PartialEq)]
pub enum SqlBindType {
    #[default]
    Question,
    Dollar,
}

/// Stores column data from database
#[derive(Debug, PartialEq)]
pub struct Column(pub(crate) String, pub(crate) Value);

impl Column {
    pub fn from_pair(name: impl Into<String>, value: impl Into<Value>) -> Self {
        Self(name.into(), value.into())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn value(&self) -> &Value {
        &self.1
    }

    pub fn inner(self) -> (String, Value) {
        (self.0, self.1)
    }
}

/// Stores row data from database
#[derive(Debug, PartialEq)]
pub struct Row(pub(crate) Vec<Column>);

impl Row {
    pub fn iter(&self) -> std::slice::Iter<'_, Column> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, idx: impl RowIndex) -> Option<&Column> {
        idx.get(self)
    }
}

impl From<Vec<Column>> for Row {
    fn from(value: Vec<Column>) -> Self {
        Self(value)
    }
}

/// Get column either by usize and &str
pub trait RowIndex {
    fn get(self, row: &Row) -> Option<&Column>;
}

impl RowIndex for usize {
    fn get(self, row: &Row) -> Option<&Column> {
        row.0.get(self)
    }
}

impl<'a> RowIndex for &'a str {
    fn get(self, row: &Row) -> Option<&Column> {
        row.0.iter().find(|c| c.name() == self)
    }
}

/// Make row to be into_iter
impl std::iter::IntoIterator for Row {
    type Item = Column;
    type IntoIter = std::vec::IntoIter<Column>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sql_builder() {
        let mut b = SqlBuilder::new_sqlite();
        b.push("SELECT * FROM table");
        assert_eq!(b.sql, "SELECT * FROM table");

        let mut b = SqlBuilder::new_mysql();
        b.push("SELECT * FROM table WHERE col = ?")
            .bind(Value::Int(Some(1)));
        assert_eq!(b.sql, "SELECT * FROM table WHERE col = ?");
        assert_eq!(b.args, vec![Value::Int(Some(1))]);

        let mut b = SqlBuilder::new_postgres();
        b.push("SELECT * FROM table WHERE col1 = ? AND col2 = ?")
            .bind(Value::Int(Some(1)))
            .bind(Value::String(Some("a".to_string())));
        assert_eq!(b.sql, "SELECT * FROM table WHERE col1 = $1 AND col2 = $2");
        assert_eq!(
            b.args,
            vec![Value::Int(Some(1)), Value::String(Some("a".to_string()))]
        );

        let mut b = SqlBuilder::new_postgres();
        let mut c = b.sub_sql();
        c.push("SELECT 1 WHERE x = ?")
            .bind(Value::Bool(Some(false)));
        b.push("SELECT * FROM table WHERE col = ? AND EXISTS (")
            .bind(Value::Int(Some(2)))
            .append(c)
            .push(")");
        assert_eq!(
            b.sql,
            "SELECT * FROM table WHERE col = $1 AND EXISTS (SELECT 1 WHERE x = $2)"
        );
        assert_eq!(b.args, vec![Value::Int(Some(2)), Value::Bool(Some(false))]);
    }

    #[test]
    fn test_row() {
        let row = Row(vec![
            Column("col1".to_string(), Value::Int(Some(1i32))),
            Column("col2".to_string(), Value::Uint(Some(2u32))),
        ]);

        assert_eq!(row.len(), 2);
        assert_eq!(
            row.get("col1"),
            Some(&Column("col1".to_string(), Value::Int(Some(1i32))))
        );
        assert_eq!(
            row.get("col2"),
            Some(&Column("col2".to_string(), Value::Uint(Some(2u32))))
        );
        assert_eq!(
            row.get(0),
            Some(&Column("col1".to_string(), Value::Int(Some(1i32))))
        );
        assert_eq!(
            row.get(1),
            Some(&Column("col2".to_string(), Value::Uint(Some(2u32))))
        );
    }
}
