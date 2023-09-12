use super::*;
/// Range of acceptable values for a typical patient, when applicable.
///
/// https://schema.org/normalRange
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum NormalRangeProperty {
    #[cfg(any(
        feature = "medical-enumeration-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalEnumeration(MedicalEnumeration),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
