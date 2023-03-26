use std::net::SocketAddr;

use axum::{Extension, Router};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::validation::CustomError;

mod authorization;
mod config;
mod controllers;
mod models;
mod services;
mod validation;

pub type Result<T, E = CustomError> = std::result::Result<T, E>;

pub async fn server(db: PgPool) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let app = api_router()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db));

    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

fn api_router() -> Router {
    Router::new().merge(controllers::user_controller::route())
}
