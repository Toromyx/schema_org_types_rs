use super::*;
/// Indicates another legislation taken into account in this consolidated legislation (which is usually the product of an editorial process that revises the legislation). This property should be used multiple times to refer to both the original version or the previous consolidated version, and to the legislations making the change.
///
/// <https://schema.org/legislationConsolidates>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum LegislationConsolidatesProperty {
    #[cfg(any(
        any(feature = "legislation-schema", feature = "pending-schema-section"),
        doc
    ))]
    Legislation(Legislation),
}
