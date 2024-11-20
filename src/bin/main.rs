use achiet::{app, database, server};
use std::env;

const ENV_PREFIX: &str = "ACHIET_";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cfg = Config::from_env()?;

    let db = database::connect(&cfg.db_url).await?;
    let state = app::AppState {
        db,
        jwt_secret: cfg.jwt_secret.clone(),
    };

    let listener = tokio::net::TcpListener::bind(cfg.addr.clone()).await?;
    log::info!("server listens on {}", cfg.addr.clone());
    axum::serve(listener, server::with_state(state.clone())).await?;

    Ok(())
}

struct Config {
    addr: String,
    db_url: String,
    jwt_secret: String,
}

impl Config {
    fn env_var(name: &str) -> Result<String, env::VarError> {
        let key = format!("{}{}", ENV_PREFIX, name);
        env::var(key.clone()).inspect_err(|_| log::error!("missing env vars {}", key))
    }

    fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            addr: Self::env_var("ADDR")?,
            db_url: Self::env_var("DB_URL")?,
            jwt_secret: Self::env_var("JWT_SECRET")?,
        })
    }
}
