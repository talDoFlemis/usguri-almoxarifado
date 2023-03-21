use std::net::SocketAddr;

use anyhow::Result;
use axum::{Extension, Router};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

mod controllers;
mod models;
mod services;
mod validation;

pub async fn server(db: PgPool) -> Result<()> {
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
