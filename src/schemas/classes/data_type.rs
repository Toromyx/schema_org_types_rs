use super::*;
/// The basic data types such as Integers, Strings, etc.
///
/// https://schema.org/DataType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DataType {}
