mod api;

use crate::app::AppState;
use axum::{
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};

pub fn with_state(state: AppState) -> axum::Router {
    axum::Router::new()
        .nest("/api", api::with_state(state))
        .fallback(fallback)
}

async fn fallback(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/').to_string();
    match WebDist::get(&path) {
        Some(_) => StaticFile(path).into_response(),
        None => StaticFile("index.html").into_response(),
    }
}

#[derive(rust_embed::Embed)]
#[folder = "web/dist/"]
struct WebDist;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match WebDist::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}
