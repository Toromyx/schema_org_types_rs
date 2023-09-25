use super::*;
/// The number of pages in the book.
///
/// <https://schema.org/numberOfPages>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NumberOfPagesProperty {
    #[cfg(any(
        any(feature = "integer-schema", feature = "general-schema-section"),
        doc
    ))]
    Integer(Integer),
}
