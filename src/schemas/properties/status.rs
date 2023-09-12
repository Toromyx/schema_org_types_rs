use super::*;
/// The status of the study (enumerated).
///
/// https://schema.org/status
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum StatusProperty {
    #[cfg(any(
        feature = "event-status-type-schema",
        feature = "general-schema-section"
    ))]
    EventStatusType(EventStatusType),
    #[cfg(any(
        feature = "medical-study-status-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalStudyStatus(MedicalStudyStatus),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
