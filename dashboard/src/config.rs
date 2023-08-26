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
}

type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug, Clone)]
pub struct Config {
    pub stats_file: String,
}

impl Config {
    pub fn new() -> ConfigResult<Config> {
        dotenv::dotenv().ok();

        Ok(Config {
            stats_file: load_env_str("STATS_FILE")?,
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
