use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
use usguri_almoxarifado::server;

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

    let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let db = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .context("could not connect to database_url")?;

    // sqlx::migrate!("./migrations")
    //     .run(&db)
    //     .await
    //     .context("could not run migrations")?;

    server(db).await?;

    Ok(())
}
