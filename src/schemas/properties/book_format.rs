use super::*;
/// The format of the book.
///
/// <https://schema.org/bookFormat>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BookFormatProperty {
    #[cfg(any(
        any(
            feature = "book-format-type-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    BookFormatType(BookFormatType),
}
