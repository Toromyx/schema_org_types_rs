use super::*;
/// The condition, complication, symptom, sign, etc. caused.
///
/// https://schema.org/causeOf
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum CauseOfProperty {
    #[cfg(any(
        feature = "medical-entity-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalEntity(MedicalEntity),
}