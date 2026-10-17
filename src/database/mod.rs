#[path = "generated/schema.rs"]
pub mod schema;
pub mod models;
pub mod database;

use self::database::{DBPool, DBConnection, get_database_pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::OnceLock;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./src/database/migrations");
static DATABASE_POOL: OnceLock<DBPool> = OnceLock::new();

fn pool() -> &'static DBPool {
    DATABASE_POOL.get_or_init(|| {
        let pool = get_database_pool();
        let mut connection = pool.get().expect("Failed to get a connection!");

        connection.run_pending_migrations(MIGRATIONS).unwrap();

        pool
    })
}

pub fn connection() -> DBConnection {
    let pool = pool();
    pool.get().expect("Failed to get a connection!")
}
