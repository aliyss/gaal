use super::{
    internals::repository::{default::GaalRepository, RepositoryError},
    provider::directory::{GaalCoreDirectory, GaalCoreDirectoryActions},
};

#[derive(Clone)]
pub struct GaalCore<GaalCoreDirectory> {
    pub directory: GaalCoreDirectory,
}

pub trait GaalCoreTrait<GCDA: GaalCoreDirectoryActions + Clone> {
    fn new(directory: GaalCoreDirectory<GCDA>) -> Self;
    fn init(&self, path: Vec<GCDA::PathItem>) -> Result<GaalRepository<GCDA>, RepositoryError>;
    fn derive_from_path(
        &self,
        path: Vec<GCDA::PathItem>,
    ) -> Result<GaalRepository<GCDA>, RepositoryError>;
}

impl<GCDA: GaalCoreDirectoryActions + Clone> GaalCoreTrait<GCDA>
    for GaalCore<GaalCoreDirectory<GCDA>>
{
    fn new(directory: GaalCoreDirectory<GCDA>) -> Self {
        Self { directory }
    }
    fn init(&self, path: Vec<GCDA::PathItem>) -> Result<GaalRepository<GCDA>, RepositoryError> {
        self.directory.init(path)
    }
    fn derive_from_path(
        &self,
        path: Vec<<GCDA>::PathItem>,
    ) -> Result<GaalRepository<GCDA>, RepositoryError> {
        self.directory.derive_from_path(path)
    }
}
