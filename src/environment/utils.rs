use crate::environment::types::RunningEnvironment;

use super::variables::EnvironmentVariable;

pub fn get_running_environment() -> RunningEnvironment {
    if cfg!(debug_assertions) {
        return RunningEnvironment::Development;
    }

    RunningEnvironment::Production
}

pub fn get_env_var(variable: EnvironmentVariable) -> String {
    let running_environment = get_running_environment();
    let variable_name = variable.as_str();
    let fallback = variable.get_development_fallback_value();

    let value: Option<String> =
        if running_environment == RunningEnvironment::Development && fallback.is_some() {
            Some(std::env::var(variable_name.clone()).unwrap_or_else(|_| fallback.unwrap().to_string()))
        } else {
            std::env::var(variable_name.clone()).ok()
        };

    value.unwrap_or_else(|| {
        if variable.is_required() {
            panic!("Environment variable {} not set!", variable_name);
        }

        String::from("")
    })
}

pub fn as_integer(value: String) -> u64 {
    value
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("Failed to cast {} into an u64!", &value))
}
