use super::{
    internals::repository::{default::GaalRepository, RepositoryError},
    provider::{
        directory::{GaalCoreDirectory, GaalCoreDirectoryActions},
        directory_object::GaalCoreDirectoryObjectsActions,
    },
};

#[derive(Clone)]
pub struct GaalCore<GaalCoreDirectory> {
    pub directory: GaalCoreDirectory,
}

pub trait GaalCoreTrait<
    GCDA: GaalCoreDirectoryActions + Clone,
    GCDOA: GaalCoreDirectoryObjectsActions<GCDA> + Clone,
>
{
    fn new(directory: GaalCoreDirectory<GCDA, GCDOA>) -> Self;
    fn init(
        &self,
        path: Vec<GCDA::PathItem>,
    ) -> Result<GaalRepository<GCDA, GCDOA>, RepositoryError>;
    fn derive_from_path(
        &self,
        path: Vec<GCDA::PathItem>,
    ) -> Result<GaalRepository<GCDA, GCDOA>, RepositoryError>;
}

impl<
        GCDA: GaalCoreDirectoryActions + Clone,
        GCDOA: GaalCoreDirectoryObjectsActions<GCDA> + Clone,
    > GaalCoreTrait<GCDA, GCDOA> for GaalCore<GaalCoreDirectory<GCDA, GCDOA>>
{
    fn new(directory: GaalCoreDirectory<GCDA, GCDOA>) -> Self {
        Self { directory }
    }
    fn init(
        &self,
        path: Vec<GCDA::PathItem>,
    ) -> Result<GaalRepository<GCDA, GCDOA>, RepositoryError> {
        self.directory.init(path)
    }
    fn derive_from_path(
        &self,
        path: Vec<<GCDA>::PathItem>,
    ) -> Result<GaalRepository<GCDA, GCDOA>, RepositoryError> {
        self.directory.derive_from_path(path)
    }
}
