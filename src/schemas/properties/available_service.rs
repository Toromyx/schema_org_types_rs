use super::*;
/// A medical service available from this provider.
///
/// https://schema.org/availableService
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AvailableServiceProperty {
    #[cfg(any(
        feature = "medical-procedure-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalProcedure(MedicalProcedure),
    #[cfg(any(
        feature = "medical-test-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalTest(MedicalTest),
    #[cfg(any(
        feature = "medical-therapy-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalTherapy(MedicalTherapy),
}
