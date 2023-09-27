use super::*;
/// <https://schema.org/studyLocation>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum StudyLocationProperty {
    #[cfg(any(
        any(
            feature = "administrative-area-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    AdministrativeArea(AdministrativeArea),
}
