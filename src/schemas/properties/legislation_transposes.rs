use super::*;
/// <https://schema.org/legislationTransposes>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum LegislationTransposesProperty {
    #[cfg(any(
        any(feature = "legislation-schema", feature = "pending-schema-section"),
        doc
    ))]
    Legislation(Legislation),
}
