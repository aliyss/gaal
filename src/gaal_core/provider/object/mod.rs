use thiserror::Error;

pub mod default;
pub mod kvlm;
pub mod tree;

#[derive(Error, Debug)]
pub enum ObjectError {
    #[error("Not a Gaal repository: `{0}`")]
    Inexistent(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Invalid data: {0}")]
    InvalidData(String),
}
