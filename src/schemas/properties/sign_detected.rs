use super::*;
/// A sign detected by the test.
///
/// https://schema.org/signDetected
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum SignDetectedProperty {
    #[cfg(any(
        feature = "medical-sign-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalSign(MedicalSign),
}
