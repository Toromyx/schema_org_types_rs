use super::*;
/// The upper value of some characteristic or property.
///
/// <https://schema.org/maxValue>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MaxValueProperty {
    #[cfg(any(
        any(feature = "number-schema", feature = "general-schema-section"),
        doc
    ))]
    Number(Number),
}
