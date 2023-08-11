use std::{
    env::{self, VarError},
    io,
};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Dot env error")]
    DotEnvError(#[from] dotenv::Error),
    #[error("IO error")]
    IOError(#[from] io::Error),
    #[error("Environment variable error")]
    EnvVarError(#[from] VarError),
    #[error("Empty variable error")]
    VarEmpty(String),
    #[error("Invalid integer `{0}`")]
    InvalidInteger(String),
    #[error("Invalid boolean `{0}`")]
    InvalidBoolean(String),
}

type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug, Clone)]
pub struct Config {
    pub check_base_url: String,
    pub send_from_email: String,
    pub service_account_file_path: String,
    pub send_to_email: String,
}

impl Config {
    pub fn new() -> ConfigResult<Config> {
        dotenv::dotenv().ok();

        Ok(Config {
            check_base_url: load_env_str("CHECK_BASE_URL")?,
            send_from_email: load_env_str("SEND_FROM_EMAIL")?,
            service_account_file_path: load_env_str("SERVICE_ACCOUNT_FILE_PATH")?,
            send_to_email: load_env_str("SEND_TO_EMAIL")?,
        })
    }
}

fn load_env_str(key: &str) -> ConfigResult<String> {
    let var = env::var(key)?;

    if var.is_empty() {
        return Err(ConfigError::VarEmpty(key.to_string()));
    }

    Ok(var)
}

fn load_env_num(key: &str) -> ConfigResult<u32> {
    let var = load_env_str(key)?;
    var.parse().map_err(|_| ConfigError::InvalidInteger(var))
}

fn load_env_bool(key: &str) -> ConfigResult<bool> {
    let var = load_env_str(key)?;
    var.parse().map_err(|_| ConfigError::InvalidBoolean(var))
}
