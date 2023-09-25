use super::*;
/// A preventative therapy used to prevent reoccurrence of the medical condition after an initial episode of the condition.
///
/// https://schema.org/secondaryPrevention
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SecondaryPreventionProperty {
    #[cfg(any(
        any(
            feature = "medical-therapy-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalTherapy(MedicalTherapy),
}
