use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonErrors {
    #[error("invalid challenge solution")]
    InvalidSolution,
}
