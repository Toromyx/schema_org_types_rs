use super::*;
/// The year an [[Accommodation]] was constructed. This corresponds to the [YearBuilt field in RESO](https://ddwiki.reso.org/display/DDW17/YearBuilt+Field).
///
/// <https://schema.org/yearBuilt>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum YearBuiltProperty {
    #[cfg(any(
        any(feature = "number-schema", feature = "general-schema-section"),
        doc
    ))]
    Number(Number),
}
