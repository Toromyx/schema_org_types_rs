use super::*;
/// <https://schema.org/educationRequirements>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum EducationRequirementsProperty {
    #[cfg(any(
        any(
            feature = "educational-occupational-credential-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    EducationalOccupationalCredential(EducationalOccupationalCredential),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
