use crate::environment::{environment, types::Database as DatabaseConfig};
use diesel::{
    PgConnection, r2d2::{ConnectionManager, Pool, PooledConnection},
};

type DBConnectionManager = ConnectionManager<PgConnection>;
pub type DBPool = Pool<DBConnectionManager>;
pub type DBConnection = PooledConnection<DBConnectionManager>;

pub fn get_database_pool() -> DBPool {
    let env = environment();
    let DatabaseConfig { user, password, uri, port, name } = &env.database;

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}",
        user,
        password,
        uri,
        port,
        name,
    );

    let manager = ConnectionManager::<PgConnection>::new(connection_string);
    Pool::builder()
        .build(manager)
        .expect("Error connecting to database!")
}
