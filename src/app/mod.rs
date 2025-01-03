//! Application state and config

use crate::database;

/// App state for global context
#[derive(Clone, Debug)]
pub struct AppState {
    pub db: database::Conn,
    pub jwt_secret: String,
}
