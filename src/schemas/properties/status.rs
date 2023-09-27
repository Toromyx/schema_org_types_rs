use super::*;
/// <https://schema.org/status>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum StatusProperty {
    #[cfg(any(
        any(
            feature = "event-status-type-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    EventStatusType(EventStatusType),
    #[cfg(any(
        any(
            feature = "medical-study-status-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalStudyStatus(MedicalStudyStatus),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
