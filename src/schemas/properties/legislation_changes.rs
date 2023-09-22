use super::*;
///
///
/// https://schema.org/legislationChanges
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LegislationChangesProperty {
    #[cfg(any(feature = "legislation-schema", feature = "pending-schema-section"))]
    Legislation(Legislation),
}