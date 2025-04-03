use std::marker::PhantomData;

use super::{
    directory::GaalCoreDirectoryActions,
    object::{default::GaalObjectAction, ObjectError},
};

pub trait GaalCoreDirectoryObjectsActionsType<GCDA>
where
    GCDA: GaalCoreDirectoryActions,
{
    type GaalBlob: GaalObjectAction<GCDA> + Clone;

    fn from_hash(hash: String) -> Result<Self::GaalBlob, ObjectError> {
        let info = hash.split("\x00").collect::<Vec<&str>>();
        let fmt = info[0].to_string();
        let data_bytes = info[2].to_string();

        match fmt.as_str() {
            "blob" => Ok(Self::GaalBlob::from_data(data_bytes.into())),
            "tree" => Ok(Self::GaalBlob::from_data(data_bytes.into())),
            "commit" => Ok(Self::GaalBlob::from_data(data_bytes.into())),
            "tag" => Ok(Self::GaalBlob::from_data(data_bytes.into())),
            _ => Err(ObjectError::Inexistent(format!(
                "Object format {} not supported",
                fmt
            ))),
        }
    }
}

pub trait GaalCoreDirectoryObjectsActions<GCDA: GaalCoreDirectoryActions>:
    GaalCoreDirectoryObjectsActionsType<GCDA> + Clone
{
}

#[derive(Clone, Debug)]
pub struct GaalCoreDirectoryObject<GCDOA: GaalCoreDirectoryActions> {
    actions: PhantomData<GCDOA>,
}
