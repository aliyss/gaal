use crate::gaal_core::provider::directory::{GaalCoreDirectory, GaalCoreDirectoryActions};

use super::RepositoryError;

pub struct GaalRepository<'a, GCDA: GaalCoreDirectoryActions + Clone> {
    pub gaal: Vec<GCDA::PathItem>,
    pub config: GCDA::Config,
    _directory: &'a GaalCoreDirectory<GCDA>,
}

impl<'a, GCDA: GaalCoreDirectoryActions + Clone> GaalRepository<'a, GCDA> {
    pub fn new(
        work_dir: Vec<GCDA::PathItem>,
        _directory: &'a GaalCoreDirectory<GCDA>,
        force: bool,
    ) -> Result<Self, RepositoryError> {
        let default_gal_dir = _directory.defaults.default_gal_dir.clone();
        let mut gaal_path = [&work_dir[..]].concat();
        gaal_path.push(default_gal_dir);

        if !force && !_directory.is_path(gaal_path.clone()) {
            // TODO: Implement join path for error message
            return Err(RepositoryError::Inexistent(format!("{:?}", gaal_path)));
        } else {
            _directory.make_path(gaal_path.clone())?;
        }

        let default_gal_config = &_directory.defaults.default_gal_config;
        let mut config_path = [&gaal_path[..]].concat();
        config_path.push(default_gal_config.clone());

        if !force && !_directory.is_path(config_path.clone()) {
            return Err(RepositoryError::Inexistent(format!("{:?}", config_path)));
        } else {
            _directory.save_data(config_path.clone(), "my config".to_string().into())?;
        }

        let config = match _directory.get_config(config_path.clone()) {
            Ok(config) => config,
            Err(e) => return Err(RepositoryError::IoError(e)),
        };

        Ok(Self {
            gaal: gaal_path,
            config,
            _directory,
        })
    }
}
