use std::marker::PhantomData;

use crate::gaal_core::internals::repository::{repository::GaalRepository, RepositoryError};

pub trait GaalCoreDirectoryActionsType {
    type PathItem: Clone + std::fmt::Debug + Into<String> + From<String>;
    type Data: Clone + std::fmt::Debug + Into<String> + From<String>;
    type Config: Clone + std::fmt::Debug + Into<String> + From<String>;
    fn make_path(path: Vec<Self::PathItem>) -> Result<(), std::io::Error>;
    fn is_path(path: Vec<Self::PathItem>) -> bool;
    fn get_path() -> Vec<Self::PathItem>;
    fn save_data(path: Vec<Self::PathItem>, data: Self::Data) -> Result<(), std::io::Error>;
    fn get_data(path: Vec<Self::PathItem>) -> Result<Self::Data, std::io::Error>;
    fn save_config(path: Vec<Self::PathItem>, config: Self::Config) -> Result<(), std::io::Error>;
    fn get_config(path: Vec<Self::PathItem>) -> Result<Self::Config, std::io::Error>;
}

#[derive(Clone, Debug)]
pub struct GaalCoreDirectoryDefaults<GCDA: GaalCoreDirectoryActionsType> {
    pub default_gal_dir: GCDA::PathItem,
    pub default_gal_config: GCDA::PathItem,
}

pub trait GaalCoreDirectoryActions: GaalCoreDirectoryActionsType + Clone {}

#[derive(Clone, Debug)]
pub struct GaalCoreDirectory<GCDA: GaalCoreDirectoryActions> {
    pub defaults: GaalCoreDirectoryDefaults<GCDA>,
    actions: PhantomData<GCDA>,
}

const DEFAULT_GAL_DIR: &str = ".gal";

impl<GCDA: GaalCoreDirectoryActions> GaalCoreDirectory<GCDA> {
    pub fn new(defaults: GaalCoreDirectoryDefaults<GCDA>) -> Self {
        Self {
            defaults,
            actions: PhantomData,
        }
    }

    pub fn init(&self, path: Vec<GCDA::PathItem>) -> Result<GaalRepository<GCDA>, RepositoryError> {
        GaalRepository::new(path, self, false)
    }

    pub fn make_path(&self, path: Vec<GCDA::PathItem>) -> Result<(), std::io::Error> {
        GCDA::make_path(path)
    }

    pub fn get_path(&self) -> Vec<GCDA::PathItem> {
        GCDA::get_path()
    }

    pub fn is_path(&self, path: Vec<GCDA::PathItem>) -> bool {
        GCDA::is_path(path)
    }

    pub fn save_data(
        &self,
        path: Vec<GCDA::PathItem>,
        data: GCDA::Data,
    ) -> Result<(), std::io::Error> {
        GCDA::save_data(path, data)
    }

    pub fn get_data(&self, path: Vec<GCDA::PathItem>) -> Result<GCDA::Data, std::io::Error> {
        GCDA::get_data(path)
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
}

impl<GCDA: GaalCoreDirectoryActions<PathItem = String> + Clone> Default
    for GaalCoreDirectory<GCDA>
{
    fn default() -> Self {
        Self {
            defaults: GaalCoreDirectoryDefaults {
                default_gal_dir: DEFAULT_GAL_DIR.to_string(),
                default_gal_config: "config".to_string(),
            },
            actions: PhantomData,
        }
    }
}
