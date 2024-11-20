mod api;

use crate::app::AppState;

pub fn with_state(state: AppState) -> axum::Router {
    axum::Router::new().nest("/api", api::with_state(state))
}
