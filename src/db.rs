//! Module for postgres db connections

pub use deadpool_postgres::Pool;

mod pool;

pub use pool::init_pool;
