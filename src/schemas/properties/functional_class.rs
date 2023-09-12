use super::*;
/// The degree of mobility the joint allows.
///
/// https://schema.org/functionalClass
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum FunctionalClassProperty {
    #[cfg(any(
        feature = "medical-entity-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalEntity(MedicalEntity),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
