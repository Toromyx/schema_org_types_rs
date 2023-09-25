use super::*;
/// Defines the month(s) of the year on which a recurring [[Event]] takes place. Specified as an [[Integer]] between 1-12. January is 1.
///
/// <https://schema.org/byMonth>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ByMonthProperty {
    #[cfg(any(
        any(feature = "integer-schema", feature = "general-schema-section"),
        doc
    ))]
    Integer(Integer),
}
