pub enum EnvironmentVariable {
    DatabaseUser,
    DatabasePassword,
    DatabaseURI,
    DatabasePort,
    DatabaseName,
    StorageAccessKey,
    StorageSecretKey,
    StorageEndpoint,
    StorageRegion,
}

impl EnvironmentVariable {
    pub fn as_str(&self) -> String {
        let str = match self {
            Self::DatabaseUser => "DATABASE_USER",
            Self::DatabasePassword => "DATABASE_PASSWORD",
            Self::DatabaseURI => "DATABASE_URI",
            Self::DatabasePort => "DATABASE_PORT",
            Self::DatabaseName => "DATABASE_NAME",
            Self::StorageAccessKey => "STORAGE_ACCESS_KEY",
            Self::StorageSecretKey => "STORAGE_SECRET_KEY",
            Self::StorageEndpoint => "STORAGE_ENDPOINT",
            Self::StorageRegion => "STORAGE_REGION",
        };
        str.to_string()
    }

    pub fn get_development_fallback_value(&self) -> Option<String> {
        let value = match self {
            Self::DatabaseUser => Some("database-development-user"),
            Self::DatabasePassword => Some("database-development-password"),
            Self::DatabaseURI => Some("localhost"),
            Self::DatabasePort => Some("5432"),
            Self::DatabaseName => Some("db"),
            Self::StorageAccessKey => Some("storage-development-user"),
            Self::StorageSecretKey => Some("storage-development-password"),
            Self::StorageEndpoint => Some("http://localhost:9000"),
            Self::StorageRegion => Some("eu-central-1"),
        };
        value.map(|value| value.to_string())
    }

    pub fn is_required(&self) -> bool {
        match self {
            Self::DatabaseUser => true,
            Self::DatabasePassword => true,
            Self::DatabaseURI => true,
            Self::DatabasePort => true,
            Self::DatabaseName => true,
            Self::StorageAccessKey => true,
            Self::StorageSecretKey => true,
            Self::StorageEndpoint => true,
            Self::StorageRegion => true,
        }
    }
}
