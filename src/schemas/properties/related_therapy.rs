use super::*;
/// A medical therapy related to this anatomy.
///
/// https://schema.org/relatedTherapy
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RelatedTherapyProperty {
    #[cfg(any(
        feature = "medical-therapy-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalTherapy(MedicalTherapy),
}
