use super::*;
/// Another gene which is a variation of this one.
///
/// https://schema.org/alternativeOf
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AlternativeOfProperty {
    #[cfg(any(feature = "gene-schema", feature = "pending-schema-section"))]
    Gene(Gene),
}
