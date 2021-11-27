use eyre::{eyre, Context, Result};
use sea_orm::{DatabaseConnection, SqlxSqliteConnector};
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct Db {
    pub orm: DatabaseConnection,
    pub pool: SqlitePool,
}

pub async fn open_db(conn_str: impl AsRef<str>) -> Result<Db> {
    let pool = SqlitePool::connect(conn_str.as_ref())
        .await
        .wrap_err_with(|| eyre!("unable to open database"))?;
    let orm = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());

    Ok(Db { pool, orm })
}
