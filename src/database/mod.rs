#[path = "generated/schema.rs"]
pub mod schema;
pub mod models;
pub mod database;

use std::sync::OnceLock;
use database::{DBPool, DBConnection, get_database_pool};

static DATABASE_POOL: OnceLock<DBPool> = OnceLock::new();

fn pool() -> &'static DBPool {
    DATABASE_POOL.get_or_init(get_database_pool)
}

pub fn connection() -> DBConnection {
    let pool = pool();
    pool.get().expect("Failed to get a connection!")
}
