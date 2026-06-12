use std::time::Duration;

use anyhow::anyhow;
use app::common::DbPool;
use log::info;
use sea_orm::{ConnectOptions, Database};

pub async fn create_db_pool() -> anyhow::Result<DbPool> {
    #[cfg(feature = "sqlx-postgres")]
    let db_conn = create_postgres_pool().await;

    #[cfg(feature = "sqlx-sqlite")]
    let db_conn = create_sqlite_pool().await;

    db_conn
}

async fn create_common_pool() -> anyhow::Result<DbPool> {
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
    .map_err(|err| anyhow!(err))
}

#[cfg(feature = "sqlx-sqlite")]
pub async fn create_sqlite_pool() -> anyhow::Result<DbPool> {
    use sea_orm::SqlxSqliteConnector;
    use sqlx::migrate::MigrateDatabase;
    use sqlx::sqlite::SqliteConnectOptions;
    use sqlx::{Sqlite, SqlitePool};
    use std::str::FromStr;

    let database_url = std::env::var("DATABASE_URL").expect("no database url specify");

    if Sqlite::database_exists(&database_url).await? {
        Ok(create_common_pool().await?)
    } else {
        let pool = SqlxSqliteConnector::from_sqlx_sqlite_pool(
            SqlitePool::connect_with(
                SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true),
            )
            .await?,
        );

        sqlx::migrate!("migrations/sqlite").run(pool.get_sqlite_connection_pool()).await.expect("migrations failed");
        Ok(create_common_pool().await?)
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn create_postgres_pool() -> anyhow::Result<DbPool> {
    let pool = create_common_pool().await?;
/*
    sqlx::migrate!("migrations/postgres")
        .run(pool.get_postgres_connection_pool())
        .await
        .expect("migrations failed");
 */
    Ok(pool)
}
