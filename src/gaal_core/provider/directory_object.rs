use std::marker::PhantomData;

use super::{
    directory::GaalCoreDirectoryActions,
    object::{default::GaalObjectAction, ObjectError},
};

#[derive(Clone, Debug)]
pub struct GaalCoreDirectoryObjectType<GCDA, GCDOAB, GCDOAC, GCDOAT, GCDOATr>
where
    GCDA: GaalCoreDirectoryActions,
    GCDOAB: GaalObjectAction<GCDA>,
    GCDOAC: GaalObjectAction<GCDA>,
    GCDOAT: GaalObjectAction<GCDA>,
    GCDOATr: GaalObjectAction<GCDA>,
{
    _gaal_core_directory_actions: Option<PhantomData<GCDA>>,
    gaal_blob: Option<GCDOAB>,
    gaal_commit: Option<GCDOAC>,
    gaal_tag: Option<GCDOAT>,
    gaal_tree: Option<GCDOATr>,
}

pub trait GaalCoreDirectoryObjectsActionsType<GCDA>
where
    GCDA: GaalCoreDirectoryActions,
{
    type GaalBlob: GaalObjectAction<GCDA> + Clone;
    type GaalCommit: GaalObjectAction<GCDA> + Clone;
    type GaalTag: GaalObjectAction<GCDA> + Clone;
    type GaalTree: GaalObjectAction<GCDA> + Clone;

    fn new_object(
        fmt: String,
        data: String,
    ) -> GaalCoreDirectoryObjectType<
        GCDA,
        Self::GaalBlob,
        Self::GaalCommit,
        Self::GaalTag,
        Self::GaalTree,
    > {
        let mut item = GaalCoreDirectoryObjectType {
            _gaal_core_directory_actions: None,
            gaal_blob: None,
            gaal_commit: None,
            gaal_tag: None,
            gaal_tree: None,
        };

        match fmt.as_str() {
            "blob" => {
                item.gaal_blob = Some(Self::GaalBlob::from_data("blob", data.into()));
            }
            "commit" => {
                item.gaal_commit = Some(Self::GaalCommit::from_data("commit", data.into()));
            }
            "tree" => {
                item.gaal_tree = Some(Self::GaalTree::from_data("tree", data.into()));
            }
            "tag" => {
                item.gaal_tag = Some(Self::GaalTag::from_data("tag", data.into()));
            }
            _ => {}
        };

        item
    }

    fn from_hash(
        hash: String,
    ) -> Result<
        GaalCoreDirectoryObjectType<
            GCDA,
            Self::GaalBlob,
            Self::GaalCommit,
            Self::GaalTag,
            Self::GaalTree,
        >,
        ObjectError,
    > {
        let info = hash.split("\x00").collect::<Vec<&str>>();
        let fmt = info[0].to_string();
        let data_bytes = info[2].to_string();

        let mut item = GaalCoreDirectoryObjectType {
            _gaal_core_directory_actions: None,
            gaal_blob: None,
            gaal_commit: None,
            gaal_tag: None,
            gaal_tree: None,
        };

        match fmt.as_str() {
            "blob" => {
                item.gaal_blob = Some(Self::GaalBlob::from_data("blob", data_bytes.into()));
            }
            "commit" => {
                item.gaal_commit = Some(Self::GaalCommit::from_data("commit", data_bytes.into()));
            }
            "tree" => {
                item.gaal_tree = Some(Self::GaalTree::from_data("tree", data_bytes.into()));
            }
            "tag" => {
                item.gaal_tag = Some(Self::GaalTag::from_data("tag", data_bytes.into()));
            }
            _ => {
                return Err(ObjectError::Inexistent(format!(
                    "Object format {} not supported",
                    fmt
                )))
            }
        };
        Ok(item)
    }

    fn hash(
        item: GaalCoreDirectoryObjectType<
            GCDA,
            Self::GaalBlob,
            Self::GaalCommit,
            Self::GaalTag,
            Self::GaalTree,
        >,
    ) -> Result<(String, String), ObjectError> {
        if let Some(gaal_blob) = &item.gaal_blob {
            gaal_blob.hash()
        } else if let Some(gaal_commit) = &item.gaal_commit {
            gaal_commit.hash()
        } else if let Some(gaal_tag) = &item.gaal_tag {
            gaal_tag.hash()
        } else if let Some(gaal_tree) = &item.gaal_tree {
            gaal_tree.hash()
        } else {
            Err(ObjectError::Inexistent("No object found".to_string()))
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
