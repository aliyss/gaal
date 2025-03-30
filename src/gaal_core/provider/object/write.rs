use crate::gaal_core::{
    internals::repository::default::GaalRepository, provider::directory::GaalCoreDirectoryActions,
};

use super::{
    default::{GaalObject, GaalObjectAction},
    ObjectError,
};

// fn object_write<GCDA, GaalObjectType>(
//     obj: GaalObject<GaalObjectType>,
//     repo: GaalRepository<GCDA>,
// ) -> Result<(), ObjectError>
// where
//     GCDA: GaalCoreDirectoryActions,
//     GaalObjectType: Clone
//         + std::fmt::Debug
//         + std::fmt::Display
//         + std::default::Default
//         + std::iter::ExactSizeIterator
//         + std::cmp::PartialEq,
// {
//     let data = GaalObjectAction::<GCDA>::serialize(&obj).unwrap();
//     let data_bytes = data.to_string().as_bytes();
//     let data_len = data.len().to_string().as_bytes();
//     let fmt_bytes = obj.fmt.as_bytes();
//     let separator = "\x00".as_bytes();
//     let all = [fmt_bytes, separator, data_len, separator, data_bytes].concat();
//     println!("{:?}", all);
//     Ok(())
// }
