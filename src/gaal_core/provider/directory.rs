use std::marker::PhantomData;

use crate::gaal_core::internals::repository::{default::GaalRepository, RepositoryError};

use super::{
    directory_config::{GaalDirectoryConfigActions, GaalDirectoryConfigSectionActions},
    directory_object::GaalCoreDirectoryObjectsActions,
};

pub trait GaalCoreDirectoryActionsType {
    type PathItem: Clone + std::fmt::Debug + Into<String> + From<String>;
    type Data: Clone + std::fmt::Debug + Into<String> + From<String>;

    type ConfigSection: GaalDirectoryConfigSectionActions + Clone + Default;
    type Config: Clone + std::fmt::Debug + GaalDirectoryConfigActions<Self::ConfigSection> + Default;
    fn make_path(path: Vec<Self::PathItem>) -> Result<(), std::io::Error>;
    fn make_entry(path: Vec<Self::PathItem>) -> Result<(), std::io::Error>;
    fn is_path(path: Vec<Self::PathItem>) -> bool;
    fn is_entry(path: Vec<Self::PathItem>) -> bool;
    fn get_path() -> Vec<Self::PathItem>;
    fn save_data(
        path: Vec<Self::PathItem>,
        data: Self::Data,
        compress: bool,
    ) -> Result<(), std::io::Error>;
    fn get_data(path: Vec<Self::PathItem>, decompress: bool) -> Result<Self::Data, std::io::Error>;
    fn is_config(path: Vec<Self::PathItem>) -> bool;
    fn save_config(path: Vec<Self::PathItem>, config: Self::Config) -> Result<(), std::io::Error>;
    fn get_config(path: Vec<Self::PathItem>) -> Result<Self::Config, std::io::Error>;
    fn hash_object_to_path(hash: String) -> Vec<Self::PathItem>;
}

#[derive(Clone, Debug)]
pub struct GaalCoreDirectoryDefaults<GCDA: GaalCoreDirectoryActionsType> {
    pub default_gal_dir: GCDA::PathItem,
    pub default_gal_config: GCDA::PathItem,
}

pub trait GaalCoreDirectoryActions: GaalCoreDirectoryActionsType + Clone {}

#[derive(Clone, Debug)]
pub struct GaalCoreDirectory<
    GCDA: GaalCoreDirectoryActions,
    GCDOA: GaalCoreDirectoryObjectsActions<GCDA>,
> {
    pub defaults: GaalCoreDirectoryDefaults<GCDA>,
    actions: PhantomData<GCDA>,
    objects: PhantomData<GCDOA>,
}

const DEFAULT_GAL_DIR: &str = ".gal";

impl<GCDA: GaalCoreDirectoryActions, GCDOA: GaalCoreDirectoryObjectsActions<GCDA>>
    GaalCoreDirectory<GCDA, GCDOA>
{
    pub fn new(defaults: GaalCoreDirectoryDefaults<GCDA>) -> Self {
        Self {
            defaults,
            actions: PhantomData,
            objects: PhantomData,
        }
    }

    pub fn init(
        &self,
        path: Vec<GCDA::PathItem>,
    ) -> Result<GaalRepository<GCDA, GCDOA>, RepositoryError> {
        GaalRepository::create(path, self)
    }

    pub fn derive_from_path(
        &self,
        path: Vec<GCDA::PathItem>,
    ) -> Result<GaalRepository<GCDA, GCDOA>, RepositoryError> {
        GaalRepository::derive_from_path(path, self)
    }

    pub fn make_path(&self, path: Vec<GCDA::PathItem>) -> Result<(), std::io::Error> {
        GCDA::make_path(path)
    }

    pub fn make_entry(&self, path: Vec<GCDA::PathItem>) -> Result<(), std::io::Error> {
        GCDA::make_entry(path)
    }

    pub fn get_path(&self) -> Vec<GCDA::PathItem> {
        GCDA::get_path()
    }

    pub fn is_path(&self, path: Vec<GCDA::PathItem>) -> bool {
        GCDA::is_path(path)
    }

    pub fn is_entry(&self, path: Vec<GCDA::PathItem>) -> bool {
        GCDA::is_entry(path)
    }

    pub fn save_data(
        &self,
        path: Vec<GCDA::PathItem>,
        data: GCDA::Data,
        compress: bool,
    ) -> Result<(), std::io::Error> {
        GCDA::save_data(path, data, compress)
    }

    pub fn get_data(
        &self,
        path: Vec<GCDA::PathItem>,
        decompress: bool,
    ) -> Result<GCDA::Data, std::io::Error> {
        GCDA::get_data(path, decompress)
    }

    pub fn is_config(&self, path: Vec<GCDA::PathItem>) -> bool {
        GCDA::is_config(path)
    }

    pub fn save_config(
        &self,
        path: Vec<GCDA::PathItem>,
        config: GCDA::Config,
    ) -> Result<(), std::io::Error> {
        GCDA::save_config(path, config)
    }

    pub fn get_config(&self, path: Vec<GCDA::PathItem>) -> Result<GCDA::Config, std::io::Error> {
        GCDA::get_config(path)
    }

    pub fn hash_object_to_path(&self, hash: String) -> Vec<GCDA::PathItem> {
        GCDA::hash_object_to_path(hash)
    }
}

impl<
        GCDA: GaalCoreDirectoryActions<PathItem = String> + Clone,
        GCDOA: GaalCoreDirectoryObjectsActions<GCDA> + Clone,
    > Default for GaalCoreDirectory<GCDA, GCDOA>
{
    fn default() -> Self {
        Self {
            defaults: GaalCoreDirectoryDefaults {
                default_gal_dir: DEFAULT_GAL_DIR.to_string(),
                default_gal_config: "config".to_string(),
            },
            actions: PhantomData,
            objects: PhantomData,
        }
    }
}
