use super::*;
/// <https://schema.org/primaryImageOfPage>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PrimaryImageOfPageProperty {
    #[cfg(any(
        any(feature = "image-object-schema", feature = "general-schema-section"),
        doc
    ))]
    ImageObject(ImageObject),
}
