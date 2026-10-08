#[derive(PartialEq)]
pub enum RunningEnvironment {
    Development,
    Production,
}

pub struct Metadata {
    pub running_environment: RunningEnvironment,
}

pub struct Database {
    pub user: String,
    pub password: String,
    pub uri: String,
    pub port: u64,
    pub name: String,
}

pub struct Environment {
    pub metadata: Metadata,
    pub database: Database,
}
