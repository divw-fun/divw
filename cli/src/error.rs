use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Invalid depth: must be between 1 and 10")]
    InvalidDepth,
}
