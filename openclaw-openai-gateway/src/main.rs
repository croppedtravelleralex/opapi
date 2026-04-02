mod app;
mod bridge;
mod config;
mod domain;
mod error;
mod governance;
mod middleware;
mod observability;
mod providers;
mod routes;
mod routing;
mod state;

use crate::{app::build_app, config::Config, state::AppState};
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = Config::from_env().expect("failed to load config");
    let state = Arc::new(AppState::new(config.clone()).await.expect("failed to init state"));
    let app = build_app(state);

    let addr: SocketAddr = format!("{}:{}", config.app_host, config.app_port)
        .parse()
        .expect("invalid bind address");

    tracing::info!(%addr, "openclaw-openai-gateway listening");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("bind failed");

    axum::serve(listener, app).await.expect("server failed");
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();
}
