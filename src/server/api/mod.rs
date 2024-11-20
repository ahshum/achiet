mod bookmark;
mod response;
mod tag;
mod user;

use response::ErrorMsg;
use user::AuthState;

use crate::app::AppState;
use axum::{
    middleware,
    routing::{get, post},
};

pub fn with_state(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/register", post(user::create))
        .route("/auth", post(user::create_jwt))
        .nest(
            "/",
            axum::Router::new()
                .nest(
                    "/user",
                    axum::Router::new()
                        .route("/", get(user::list).post(user::create))
                        .route(
                            "/:user_id",
                            get(user::find).put(user::update).delete(user::delete),
                        ),
                )
                .nest(
                    "/bookmark",
                    axum::Router::new()
                        .route("/", get(bookmark::list).post(bookmark::create))
                        .route(
                            "/:bookmark_id",
                            get(bookmark::find)
                                .put(bookmark::update)
                                .delete(bookmark::delete),
                        ),
                )
                .nest(
                    "/tag",
                    axum::Router::new()
                        .route("/", get(tag::list).post(tag::create))
                        .route(
                            "/:tag_id",
                            get(tag::find).put(tag::update).delete(tag::delete),
                        ),
                )
                .layer(middleware::from_fn(user::authenticate)),
        )
        .layer(axum::Extension(state))
}
