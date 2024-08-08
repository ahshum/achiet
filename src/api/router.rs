use super::state::AuthenticationState;
use crate::app::AppState;

#[derive(Clone, Default)]
pub struct Config {
    pub jwt_secret: String,
}

pub fn new_handler(app_state: AppState, config: Config) -> axum::Router {
    use axum::routing::{get, post};

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods(tower_http::cors::Any)
        .allow_origin(tower_http::cors::Any)
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::ACCEPT,
            axum::http::header::CONTENT_TYPE,
        ]);

    let mut router = axum::Router::new()
        .route("/metatag", get(super::metatag::fetch))
        .route("/register", post(super::user::create))
        .route("/auth", post(super::user::create_token))
        .nest(
            "/",
            axum::Router::new()
                .nest(
                    "/user",
                    axum::Router::new()
                        .route("/", get(super::user::list).post(super::user::create))
                        .route(
                            "/:user_id",
                            get(super::user::find)
                                .put(super::user::update)
                                .delete(super::user::delete),
                        ),
                )
                .nest(
                    "/bookmark",
                    axum::Router::new()
                        .route(
                            "/",
                            get(super::bookmark::list).post(super::bookmark::create),
                        )
                        .route(
                            "/:bookmark_id",
                            get(super::bookmark::find)
                                .put(super::bookmark::update)
                                .delete(super::bookmark::delete),
                        ),
                )
                .nest(
                    "/tag",
                    axum::Router::new()
                        .route("/", get(super::tag::list).post(super::tag::create))
                        .route(
                            "/:tag_id",
                            get(super::tag::find)
                                .put(super::tag::update)
                                .delete(super::tag::delete),
                        ),
                )
                .route_layer(axum::middleware::from_fn(super::user::authenticate)),
        );

    router = router
        .layer(axum::Extension(app_state))
        .layer(axum::Extension(config))
        .layer(axum::Extension(AuthenticationState::Guest))
        .layer(cors);

    router
}
