use std::marker::PhantomData;

use crate::gaal_core::internals::repository::{repository::GaalRepository, RepositoryError};

pub trait GaalCoreDirectoryActionsType {
    type PathItem: Clone + std::fmt::Debug;
    fn make_dir(path: Vec<Self::PathItem>) -> Result<(), std::io::Error>;
    fn is_dir(path: Vec<Self::PathItem>) -> bool;
    fn get_dir() -> Vec<Self::PathItem>;
}

pub trait GaalCoreDirectoryActions: GaalCoreDirectoryActionsType + Clone {}

#[derive(Clone, Debug)]
pub struct GaalCoreDirectory<GCDA: GaalCoreDirectoryActions> {
    pub default_gal_dir: GCDA::PathItem,
    actions: PhantomData<GCDA>,
}

const DEFAULT_GAL_DIR: &str = ".gal";

impl<GCDA: GaalCoreDirectoryActions> GaalCoreDirectory<GCDA> {
    pub fn new(default_gal_dir: GCDA::PathItem) -> Self {
        Self {
            default_gal_dir,
            actions: PhantomData,
        }
    }

    pub fn init(&self, path: Vec<GCDA::PathItem>) -> Result<GaalRepository<GCDA>, RepositoryError> {
        GaalRepository::new(path, self, false)
    }

    pub fn make_dir(&self, path: Vec<GCDA::PathItem>) -> Result<(), std::io::Error> {
        GCDA::make_dir(path)
    }

    pub fn get_dir(&self) -> Vec<GCDA::PathItem> {
        GCDA::get_dir()
    }

    pub fn is_dir(&self, path: Vec<GCDA::PathItem>) -> bool {
        GCDA::is_dir(path)
    }
}

impl<GCDA: GaalCoreDirectoryActions<PathItem = String> + Clone> Default
    for GaalCoreDirectory<GCDA>
{
    fn default() -> Self {
        Self {
            default_gal_dir: DEFAULT_GAL_DIR.to_string(),
            actions: PhantomData,
        }
    }
}
