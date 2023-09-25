use super::*;
/// The status of the study (enumerated).
///
/// <https://schema.org/status>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
