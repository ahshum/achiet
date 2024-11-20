//! Application state and config

use crate::database;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: database::Conn,
    pub jwt_secret: String,
}
