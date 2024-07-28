use crate::{
    api::{self, state::AuthenticationState},
    app::AppState,
};

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
        ]);

    let mut router = axum::Router::new()
        .route("/register", post(api::user::create))
        .route("/auth", post(api::user::create_token))
        .nest(
            "/",
            axum::Router::new()
                .nest(
                    "/user",
                    axum::Router::new()
                        .route("/", get(api::user::list).post(api::user::create))
                        .route(
                            "/:user_id",
                            get(api::user::find)
                                .put(api::user::update)
                                .delete(api::user::delete),
                        ),
                )
                .nest(
                    "/bookmark",
                    axum::Router::new()
                        .route("/", get(api::bookmark::list).post(api::bookmark::create))
                        .route(
                            "/:bookmark_id",
                            get(api::bookmark::find)
                                .put(api::bookmark::update)
                                .delete(api::bookmark::delete),
                        ),
                )
                .nest(
                    "/tag",
                    axum::Router::new()
                        .route("/", get(api::tag::list).post(api::tag::create))
                        .route(
                            "/:tag_id",
                            get(api::tag::find)
                                .put(api::tag::update)
                                .delete(api::tag::delete),
                        ),
                )
                .route_layer(axum::middleware::from_fn(api::user::authenticate)),
        );

    router = router
        .layer(axum::Extension(app_state))
        .layer(axum::Extension(config))
        .layer(axum::Extension(AuthenticationState::Guest))
        .layer(cors);

    router
}
