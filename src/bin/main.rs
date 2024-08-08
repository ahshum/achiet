use achiet::{api, app::AppState, database, taskqueue};

#[tokio::main]
async fn main() {
    let dsn = "sqlite://app.db";
    let worker_count = 4u32;

    env_logger::init();

    let db = database::connect(dsn.to_string()).await.unwrap();
    let (dispatcher, worker) = taskqueue::channel();

    let app_state = AppState::new(db, dispatcher.clone());
    let rt = tokio::runtime::Runtime::new().unwrap();

    for i in 0..worker_count {
        let app_state_clone = app_state.clone();
        let worker_clone = worker.clone();
        tokio::spawn(async move {
            let _ = worker_clone.work(app_state_clone).await;
        });
    }

    let api_handler = api::new_handler(
        app_state.clone(),
        api::Config {
            jwt_secret: "secret".to_string(),
        },
    );
    let serve_web_index = tower_http::services::ServeFile::new("web/dist/index.html");
    let serve_web_dir = tower_http::services::ServeDir::new("web/dist");
    let router = axum::Router::new()
        .nest("/api", api_handler)
        .nest_service("/", serve_web_dir.not_found_service(serve_web_index));

    let address = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    log::info!("listening on {}", address);
    axum::serve(listener, router).await.unwrap();
}
