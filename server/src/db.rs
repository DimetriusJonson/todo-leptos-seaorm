use std::time::Duration;

use app::common::DbPool;
use log::info;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};
use sea_orm::{ConnectOptions, Database, DbErr};

/*
#[cfg(feature = "ssr")]
pub async fn create_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .min_connections(1)
        .max_connections(3)
        .connect(database_url.as_str())
        .await
        .expect("could not connect to database_url");

    sqlx::migrate!("./migrations/postgres")
        .run(&pool)
        .await
        .expect("migrations failed");

    pool
}
 */

 pub async fn create_db_pool() -> Result<DbPool, DbErr> {
    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");
    info!("database_url={}", database_url);

    Database::connect(
        ConnectOptions::new(database_url)
            .max_connections(3)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(600))
            .max_lifetime(Duration::from_secs(600))
            .sqlx_logging(true)
            .clone(),
    )
    .await
}

pub async fn create_sqlite_pool() -> anyhow::Result<DbPool> {
    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");

    if Sqlite::database_exists(&database_url).await? {
        Ok(create_db_pool().await?)
    } else {
        let db = SqlitePool::connect(&database_url).await?;
        sqlx::migrate!("migrations/sqlite").run(&db).await.expect("migrations failed");
        Ok(create_db_pool().await?)
    }
}
