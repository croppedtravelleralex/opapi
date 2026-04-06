mod app;
mod auth;
mod config;
mod error;
mod routes;
mod store;

use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env()?;
    let store = store::AccountStore::open("data/gateway.db")?;
    let seeded_accounts = store.list_accounts()?;
    tracing::info!(count = seeded_accounts.len(), "account store initialized");

    let listener = tokio::net::TcpListener::bind(config.bind_addr()).await?;
    tracing::info!(addr = %config.bind_addr(), title = %config.api_title, "gateway listening");

    axum::serve(listener, app::build_router(config)).await?;
    Ok(())
}
