use std::error::Error;

use crate::gaal_core::provider::directory::{GaalCoreDirectory, GaalCoreDirectoryActions};

use super::RepositoryError;

pub struct GaalRepository<'a, GCDA: GaalCoreDirectoryActions + Clone> {
    pub work_dir: Vec<GCDA::PathItem>,
    _directory: &'a GaalCoreDirectory<GCDA>,
}

impl<'a, GCDA: GaalCoreDirectoryActions + Clone> GaalRepository<'a, GCDA> {
    pub fn new(
        work_dir: Vec<GCDA::PathItem>,
        _directory: &'a GaalCoreDirectory<GCDA>,
        force: bool,
    ) -> Result<Self, RepositoryError> {
        let default_gal_dir = _directory.default_gal_dir.clone();
        let mut full_path = [&work_dir[..]].concat();
        full_path.push(default_gal_dir.clone());

        if !force && !_directory.is_dir(full_path.clone()) {
            // TODO: Implement join path for error message
            return Err(RepositoryError::Inexistent(format!("{:?}", full_path)));
        } else {
            println!("Creating Gaal repository at {:?}", full_path);
        }

        Ok(Self {
            work_dir,
            _directory,
        })
    }
}
