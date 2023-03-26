use anyhow::{Context, Result};
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use usguri_almoxarifado::{config::Config, server};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "usguri_almoxarifado=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::parse();

    let db = PgPoolOptions::new()
        .max_connections(100)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    // sqlx::migrate!("./migrations")
    //     .run(&db)
    //     .await
    //     .context("could not run migrations")?;

    server(db, config).await?;

    Ok(())
}
