use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
use anyhow::Context;

pub async fn init_pool() -> anyhow::Result<Pool> {
    let mut cfg = Config::new();
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let user = std::env::var("DB_USER").context("DB_USER missing")?;
    let password = std::env::var("DB_PASSWORD").context("DB_PASSWORD missing")?;
    let host = std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".into());
    let db_name = std::env::var("DB_NAME").context("DB_NAME missing")?;
    let db_type = std::env::var("DB_TYPE").unwrap_or_else(|_| "postgres".into());
    let url = format!("{db_type}://{user}:{password}@{host}/{db_name}");
    cfg.url = Some(url);

    let pool = cfg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
    Ok(pool)
}
