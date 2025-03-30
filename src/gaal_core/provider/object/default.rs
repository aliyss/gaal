use crate::gaal_core::{
    internals::repository::default::GaalRepository, provider::directory::GaalCoreDirectoryActions,
};

use super::ObjectError;

pub trait GaalObjectAction<GCDA: GaalCoreDirectoryActions> {
    type Data: Clone + std::fmt::Debug + Default + ExactSizeIterator;
    type Serialized: Clone + std::fmt::Debug + Default + ExactSizeIterator + ToString;

    fn new() -> Self;
    fn from_serialized(data: Self::Serialized) -> Self;
    fn from_data(data: Self::Data) -> Self;
    fn serialize(&self) -> Result<Self::Serialized, ObjectError>;
    fn serialize_data(data: Self::Data) -> Result<Self::Serialized, ObjectError>;
    fn deserialize(&self) -> Result<Self::Data, ObjectError>;
    fn deserialize_data(data: Self::Serialized) -> Result<Self::Data, ObjectError>;
}

pub struct GaalObject<GaalObjectType>
where
    GaalObjectType: Clone + std::fmt::Debug + Default + ExactSizeIterator,
{
    pub fmt: String,
    pub data: GaalObjectType,
}

impl<GCDA, GaalObjectType> GaalObjectAction<GCDA> for GaalObject<GaalObjectType>
where
    GCDA: GaalCoreDirectoryActions,
    GaalObjectType: Clone + std::fmt::Debug + Default + ExactSizeIterator + std::fmt::Display,
{
    type Data = GaalObjectType;
    type Serialized = Self::Data;

    fn new() -> Self {
        Self {
            fmt: "blob".to_string(),
            data: GaalObjectType::default(),
        }
    }

    fn from_serialized(data: Self::Serialized) -> Self {
        let deserialized =
            <GaalObject<GaalObjectType> as GaalObjectAction<GCDA>>::deserialize_data(data).unwrap();
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
