use crate::gaal_core::provider::{
    directory::{GaalCoreDirectory, GaalCoreDirectoryActions},
    directory_object::{GaalCoreDirectoryObjectType, GaalCoreDirectoryObjectsActions},
    object::{default::GaalObjectAction, ObjectError},
};

use super::RepositoryError;

pub struct GaalRepository<
    'a,
    GCDA: GaalCoreDirectoryActions + Clone,
    GCDOA: GaalCoreDirectoryObjectsActions<GCDA> + Clone,
> {
    pub gaal: Vec<GCDA::PathItem>,
    pub config: GCDA::Config,
    _directory: &'a GaalCoreDirectory<GCDA, GCDOA>,
}

impl<
        'a,
        GCDA: GaalCoreDirectoryActions + Clone,
        GCDOA: GaalCoreDirectoryObjectsActions<GCDA> + Clone,
    > GaalRepository<'a, GCDA, GCDOA>
{
    pub fn new(
        work_dir: Vec<GCDA::PathItem>,
        _directory: &'a GaalCoreDirectory<GCDA, GCDOA>,
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

        let default_gal_config = _directory.defaults.default_gal_config.clone();
        let config_path = {
            let mut path = gaal_path.clone();
            path.push(default_gal_config.clone());
            path
        };

        if !force && !_directory.is_config(config_path.clone()) {
            return Err(RepositoryError::Inexistent(format!("{:?}", config_path)));
        } else {
            _directory.save_config(config_path.clone(), GCDA::Config::default())?;
        }

        let branch_path: Vec<GCDA::PathItem> = {
            let mut path = gaal_path.clone();
            path.push("branches".to_string().into());
            path
        };

        if !_directory.is_path(branch_path.clone()) {
            _directory.make_path(branch_path)?;
        }

        let objects_path: Vec<GCDA::PathItem> = {
            let mut objects_path = gaal_path.clone();
            objects_path.push("objects".to_string().into());
            objects_path
        };

        if !_directory.is_path(objects_path.clone()) {
            _directory.make_path(objects_path)?;
        }

        let refs_path: Vec<GCDA::PathItem> = {
            let mut refs_path = gaal_path.clone();
            refs_path.push("refs".to_string().into());
            refs_path
        };

        let heads_path: Vec<GCDA::PathItem> = {
            let mut heads_path = refs_path.clone();
            heads_path.push("heads".to_string().into());
            heads_path
        };

        let tags_path: Vec<GCDA::PathItem> = {
            let mut tags_path = refs_path.clone();
            tags_path.push("tags".to_string().into());
            tags_path
        };

        if !_directory.is_path(refs_path.clone()) {
            _directory.make_path(refs_path)?;
        }

        if !_directory.is_path(heads_path.clone()) {
            _directory.make_path(heads_path)?;
        }

        if !_directory.is_path(tags_path.clone()) {
            _directory.make_path(tags_path)?;
        }

        let description_path: Vec<GCDA::PathItem> = {
            let mut description_path = gaal_path.clone();
            description_path.push("description".to_string().into());
            description_path
        };

        if !_directory.is_path(description_path.clone()) {
            _directory.save_data(
                description_path,
                "Unnamed repository; edit this file 'description' to name the repository."
                    .to_string()
                    .into(),
                false,
            )?;
        }

        let head_path: Vec<GCDA::PathItem> = {
            let mut head_path = gaal_path.clone();
            head_path.push("HEAD".to_string().into());
            head_path
        };

        if !_directory.is_path(head_path.clone()) {
            _directory.save_data(
                head_path,
                "ref: refs/heads/master".to_string().into(),
                false,
            )?;
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

    pub fn create(
        work_dir: Vec<GCDA::PathItem>,
        _directory: &'a GaalCoreDirectory<GCDA, GCDOA>,
    ) -> Result<Self, RepositoryError> {
        Self::new(work_dir, _directory, true)
    }

    pub fn derive_from_path(
        work_dir: Vec<GCDA::PathItem>,
        _directory: &'a GaalCoreDirectory<GCDA, GCDOA>,
    ) -> Result<Self, RepositoryError> {
        let default_gal_dir = _directory.defaults.default_gal_dir.clone();
        let mut gaal_path = [&work_dir[..]].concat();
        gaal_path.push(default_gal_dir);

        if _directory.is_path(gaal_path.clone()) {
            return Self::new(work_dir, _directory, false);
        }

        let parent = {
            let mut parent = work_dir.clone();
            parent.pop();
            parent
        };

        if work_dir.is_empty() && parent.is_empty() {
            return Err(RepositoryError::Inexistent(
                "No .gal directory.".to_string(),
            ));
        }

        Self::derive_from_path(parent, _directory)
    }

    pub fn object_write(
        &self,
        obj: GaalCoreDirectoryObjectType<
            GCDA,
            GCDOA::GaalBlob,
            GCDOA::GaalCommit,
            GCDOA::GaalTag,
            GCDOA::GaalTree,
        >,
    ) -> Result<String, ObjectError>
    where
        GCDA: GaalCoreDirectoryActions,
    {
        let (hash, result) = GCDOA::hash(obj.clone())?;

        let path = self._directory.hash_object_to_path(hash.clone());
        let object_path = {
            let mut objects_path = self.gaal.clone();
            objects_path.push("objects".to_string().into());
            for opath in path.iter() {
                objects_path.push(opath.clone());
            }
            objects_path
        };

        if !self._directory.is_entry(object_path.clone()) {
            self._directory.make_entry(object_path.clone())?;
        }

        self._directory
            .save_data(object_path, result.into(), true)?;

        Ok(hash)
    }

    pub fn object_read(
        &self,
        hash: String,
    ) -> Result<
        GaalCoreDirectoryObjectType<
            GCDA,
            GCDOA::GaalBlob,
            GCDOA::GaalCommit,
            GCDOA::GaalTag,
            GCDOA::GaalTree,
        >,
        ObjectError,
    >
    where
        GCDA: GaalCoreDirectoryActions,
    {
        let path = self._directory.hash_object_to_path(hash);
        let object_path = {
            let mut objects_path = self.gaal.clone();
            objects_path.push("objects".to_string().into());
            for opath in path.iter() {
                objects_path.push(opath.clone());
            }
            objects_path
        };

        if !self._directory.is_entry(object_path.clone()) {
            return Err(ObjectError::Inexistent(format!("{:?}", object_path)));
        }

        let data = self._directory.get_data(object_path, true)?;

        GCDOA::from_hash(data.into())
    }
}
