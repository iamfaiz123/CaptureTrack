use derive_more::Display;
use thiserror::Error;

#[derive(Error, Debug, Display)]
pub enum CoreError {
    ConfigBuilderError(#[from] config::ConfigError),
    ApplicationError(#[from] anyhow::Error),
}
