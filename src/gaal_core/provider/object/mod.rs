use thiserror::Error;

pub mod default;

#[derive(Error, Debug)]
pub enum ObjectError {
    #[error("Not a Gaal repository: `{0}`")]
    Inexistent(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
