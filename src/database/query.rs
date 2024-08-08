use super::value::{Value, Values};

#[derive(Debug, Clone)]
pub struct Column(pub String, pub Value);

#[derive(Debug, Clone)]
pub struct Row {
    pub columns: Vec<Column>,
}

impl Row {
    pub fn len(&self) -> usize {
        self.columns.len()
    }

    pub fn column_name(&self, index: ColumnIndex) -> Result<String, ()> {
        if let Some(c) = index.column_index(self) {
            Ok(c.0.clone())
        } else {
            Err(())
        }
    }

    pub fn try_get(&self, index: ColumnIndex) -> Result<Value, ()> {
        if let Some(c) = index.column_index(self) {
            Ok(c.1.clone())
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub enum ColumnIndex {
    Int(usize),
    Str(String),
}

impl Into<ColumnIndex> for usize {
    fn into(self) -> ColumnIndex {
        ColumnIndex::Int(self)
    }
}

impl Into<ColumnIndex> for String {
    fn into(self) -> ColumnIndex {
        ColumnIndex::Str(self)
    }
}

impl Into<ColumnIndex> for &str {
    fn into(self) -> ColumnIndex {
        ColumnIndex::Str(self.to_string())
    }
}

impl ColumnIndex {
    fn column_index(self, row: &Row) -> Option<Column> {
        match self {
            Self::Str(s) => {
                if let Some(c) = row.columns.iter().find(|c| c.0 == s) {
                    Some(c.clone())
                } else {
                    None
                }
            }
            Self::Int(i) => {
                if let Some(c) = row.columns.get(i) {
                    Some(c.clone())
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Query {
    pub(crate) stmt: Vec<String>,
    pub(crate) args: Values,
    pub(crate) sep: String,
}

impl Query {
    pub fn new() -> Self {
        Self {
            stmt: Vec::new(),
            args: Values::default(),
            sep: "".to_string(),
        }
    }

    pub fn set_separator(&mut self, sep: &str) {
        self.sep = sep.to_string();
    }

    pub fn append(&mut self, other: Self) -> &mut Self {
        let (stmt, mut values) = other.build();
        self.stmt.push(stmt);
        self.args.append(&mut values);
        self
    }

    pub fn push_str(&mut self, stmt: &str) -> &mut Self {
        self.stmt.push(stmt.to_string());
        self
    }

    pub fn bind(&mut self, value: Value) -> &mut Self {
        self.args.push(value);
        self
    }

    pub fn build(self) -> (String, Values) {
        (self.stmt.join(&self.sep), self.args)
    }

    pub fn is_empty(&self) -> bool {
        self.stmt.len() == 0 && self.args.0.len() == 0
    }
}

impl Into<Query> for &str {
    fn into(self) -> Query {
        let mut query = Query::new();
        query.push_str(self);
        query
    }
}

impl Into<Query> for String {
    fn into(self) -> Query {
        let mut query = Query::new();
        query.push_str(self.as_str());
        query
    }
}
