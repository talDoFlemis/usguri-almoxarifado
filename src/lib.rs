use std::{net::SocketAddr, sync::Arc};

use axum::{Extension, Router};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::{config::Config, validation::CustomError};

mod authorization;
pub mod config;
mod controllers;
mod models;
mod services;
mod validation;

pub type Result<T, E = CustomError> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    config: Arc<Config>,
}

pub async fn server(db: PgPool, cfg: Config) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let state = AppState {
        db,
        config: Arc::new(cfg),
    };
    let app = api_router()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(state));

    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn api_router() -> Router {
    Router::new().merge(controllers::user_controller::route())
}
