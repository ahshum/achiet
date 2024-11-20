use axum::{body, extract::Json, http, response};

/// Error message response
pub struct ErrorMsg(pub u16, pub String);

impl response::IntoResponse for ErrorMsg {
    fn into_response(self) -> response::Response<body::Body> {
        let mut body = serde_json::Map::new();
        body.extend(vec![("error".into(), self.1.into())]);
        let mut res = Json(body).into_response();
        *res.status_mut() = http::StatusCode::from_u16(self.0).unwrap();
        res
    }
}
