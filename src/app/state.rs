use crate::database::{Database, Query};

#[derive(Clone)]
pub struct AppState {
    db: Database,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn database(&self) -> &Database {
        &self.db
    }

    pub fn new_query(&self) -> Query {
        Query::new()
    }
}
