use super::*;
/// <https://schema.org/signDetected>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SignDetectedProperty {
    #[cfg(any(
        any(
            feature = "medical-sign-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalSign(MedicalSign),
}
