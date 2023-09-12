use super::*;
/// A preventative therapy used to prevent reoccurrence of the medical condition after an initial episode of the condition.
///
/// https://schema.org/secondaryPrevention
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SecondaryPreventionProperty {
    #[cfg(any(
        feature = "medical-therapy-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalTherapy(MedicalTherapy),
}
