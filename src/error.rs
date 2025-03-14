use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Seed bundle error")]
    SeedBundle(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Setup error: {0}")]
    Setup(String),
}

impl From<color_eyre::Report> for CliError {
    fn from(err: color_eyre::Report) -> Self {
        CliError::Setup(err.to_string())
    }
}
