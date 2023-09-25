use super::*;
/// Another gene which is a variation of this one.
///
/// <https://schema.org/alternativeOf>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AlternativeOfProperty {
    #[cfg(any(any(feature = "gene-schema", feature = "pending-schema-section"), doc))]
    Gene(Gene),
}
