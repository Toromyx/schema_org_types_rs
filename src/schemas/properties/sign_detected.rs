use super::*;
/// A sign detected by the test.
///
/// <https://schema.org/signDetected>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
