use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use usguri_almoxarifado::server;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL not set")?;
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await
        .context("could not connect to database_url")?;

    server(db).await?;

    Ok(())
}
