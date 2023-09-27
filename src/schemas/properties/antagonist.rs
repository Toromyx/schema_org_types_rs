use super::*;
/// <https://schema.org/antagonist>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AntagonistProperty {
    #[cfg(any(
        any(feature = "muscle-schema", feature = "health-lifesci-schema-section"),
        doc
    ))]
    Muscle(Muscle),
}
