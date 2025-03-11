use thiserror::Error;

pub mod repository;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Not a Gaal repository at `{0}`")]
    Inexistent(String),
}
