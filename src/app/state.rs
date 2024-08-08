use crate::{
    database::{Database, Query},
    taskqueue::{Dispatcher, Task},
};

#[derive(Clone)]
pub struct AppState {
    db: Database,
    dispatcher: Dispatcher,
}

impl AppState {
    pub fn new(db: Database, dispatcher: Dispatcher) -> Self {
        Self { db, dispatcher }
    }

    pub fn database(&self) -> &Database {
        &self.db
    }

    pub fn new_query(&self) -> Query {
        Query::new()
    }

    pub fn dispatcher(&self) -> &Dispatcher {
        &self.dispatcher
    }
}
