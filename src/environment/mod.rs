pub mod types;
mod variables;
mod utils;

use types::*;
use variables::EnvironmentVariable::*;
use utils::{get_env_var, get_running_environment, as_integer};
use std::sync::OnceLock;

static ENVIRONMENT: OnceLock<Environment> = OnceLock::new();

pub fn environment() -> &'static Environment {
    ENVIRONMENT.get_or_init(|| {
        dotenvy::dotenv().ok();

        Environment {
            metadata: Metadata {
                running_environment: get_running_environment(),
            },
            database: Database {
                user: get_env_var(DatabaseUser),
                password: get_env_var(DatabasePassword),
                uri: get_env_var(DatabaseURI),
                port: as_integer(get_env_var(DatabasePort)),
                name: get_env_var(DatabaseName),
            },
        }
    })
}
