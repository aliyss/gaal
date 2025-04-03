use crate::gaal_core::provider::directory::GaalCoreDirectoryActions;
use sha2::{Digest, Sha256};

use super::ObjectError;

pub trait GaalObjectAction<GCDA: GaalCoreDirectoryActions> {
    type Data: Clone + std::fmt::Debug + Default + From<String> + Into<String>;
    type Serialized: Clone + std::fmt::Debug + Default + ToString;

    fn new() -> Self;
    fn from_serialized(data: Self::Serialized) -> Self;
    fn from_data(data: Self::Data) -> Self;
    fn serialize(&self) -> Result<Self::Serialized, ObjectError>;
    fn serialize_data(data: Self::Data) -> Result<Self::Serialized, ObjectError>;
    fn deserialize(&self) -> Result<Self::Data, ObjectError>;
    fn deserialize_data(data: Self::Serialized) -> Result<Self::Data, ObjectError>;

    fn hash(&self) -> Result<(String, String), ObjectError> {
        let data = self.serialize()?;
        let data_bytes = data.to_string();
        let data_len = data_bytes.len().to_string();
        let fmt_bytes = "blob".to_string();
        let separator = "\x00".to_string();
        let all = [
            fmt_bytes,
            separator.clone(),
            data_len,
            separator,
            data_bytes,
        ]
        .concat();
        let mut hasher = Sha256::new();
        hasher.update(&all);
        let hash = hasher.finalize();
        Ok((format!("{:x}", hash), all))
    }
}

#[derive(Clone, Debug)]
pub struct GaalObject<GaalObjectType> {
    pub fmt: String,
    pub data: GaalObjectType,
}

impl<GCDA> GaalObjectAction<GCDA> for GaalObject<String>
where
    GCDA: GaalCoreDirectoryActions,
{
    type Data = String;
    type Serialized = String;

    fn new() -> Self {
        Self {
            fmt: "blob".to_string(),
            data: String::new(),
        }
    }

    fn from_serialized(data: Self::Serialized) -> Self {
        let deserialized =
            <GaalObject<String> as GaalObjectAction<GCDA>>::deserialize_data(data).unwrap();
        Self {
            fmt: "blob".to_string(),
            data: deserialized,
        }
    }

    fn from_data(data: Self::Data) -> Self {
        Self {
            fmt: "blob".to_string(),
            data,
        }
    }

    fn serialize(&self) -> Result<Self::Serialized, ObjectError> {
        Ok(self.data.clone())
    }

    fn serialize_data(data: Self::Data) -> Result<Self::Serialized, ObjectError> {
        Ok(data)
    }

    fn deserialize(&self) -> Result<Self::Data, ObjectError> {
        Ok(self.data.clone())
    }

    fn deserialize_data(data: Self::Serialized) -> Result<Self::Data, ObjectError> {
        Ok(data)
    }
}
