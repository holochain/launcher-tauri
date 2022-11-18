use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum HcLaunchError {

    #[error("Specified UI path \"{0}\" does not exist.")]
    UiPathDoesNotExist(String),

    /// anything else
    #[error("Unknown error: {0}")]
    MiscError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// HcBundle Result type.
pub type HcLaunchResult<T> = Result<T, HcLaunchError>;